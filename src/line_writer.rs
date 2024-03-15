use crate::byte_to_color::ByteToColor;
use std::io::prelude::*;

pub struct LineWriter {
    writer: std::io::BufWriter<std::io::Stdout>,
    hex: [[u8; 3]; 256],
    byte_to_color: ByteToColor,
    max_bytes_per_line: usize,
    byte_counter: usize,
}

impl LineWriter {
    // https://de.wikipedia.org/wiki/Codepage_437
    #[rustfmt::skip]
    const CODE_PAGE_437: [&'static str; 256] = [
        "⋄", "☺", "☻", "♥", "♦", "♣", "♠", "•", "◘", "○", "◙", "♂", "♀", "♪", "♫", "☼", // 00-0f
        "►", "◄", "↕", "‼", "¶", "§", "▬", "↨", "↑", "↓", "→", "←", "∟", "↔", "▲", "▼", // 10-1f
        " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", // 20-2f
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", // 30-3f
        "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", // 40-4f
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", // 50-5f
        "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", // 60-6f
        "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "⌂", // 70-7f
        "Ç", "ü", "é", "â", "ä", "à", "å", "ç", "ê", "ë", "è", "ï", "î", "ì", "Ä", "Å", // 80-8f
        "É", "æ", "Æ", "ô", "ö", "ò", "û", "ù", "ÿ", "Ö", "Ü", "¢", "£", "¥", "₧", "ƒ", // 90-9f
        "á", "í", "ó", "ú", "ñ", "Ñ", "ª", "º", "¿", "⌐", "¬", "½", "¼", "¡", "«", "»", // a0-af
        "░", "▒", "▓", "│", "┤", "╡", "╢", "╖", "╕", "╣", "║", "╗", "╝", "╜", "╛", "┐", // b0-bf
        "└", "┴", "┬", "├", "─", "┼", "╞", "╟", "╚", "╔", "╩", "╦", "╠", "═", "╬", "╧", // c0-cf
        "╨", "╤", "╥", "╙", "╘", "╒", "╓", "╫", "╪", "┘", "┌", "█", "▄", "▌", "▐", "▀", // d0-df
        "α", "ß", "Γ", "π", "Σ", "σ", "µ", "τ", "Φ", "Θ", "Ω", "δ", "∞", "φ", "ε", "∩", // e0-ef
        "≡", "±", "≥", "≤", "⌠", "⌡", "÷", "≈", "°", "∙", "·", "√", "ⁿ", "²", "■", "ﬀ", // f0-ff
    ];

    const SPACE: &'static [u8] = b" ";
    const NEWLINE: &'static [u8] = b"\n";
    const HEX_SPACE: [u8; 3] = [b' '; 3];
    const COLOR_RESET: &'static str = "[0m";

    pub fn new(max_bytes_per_line: usize) -> Self {
        let mut hex = [[0u8; 3]; 256];
        for i in 0..256 {
            let hex_chars = b"0123456789abcdef";
            hex[i][0] = hex_chars[i >> 4];
            hex[i][1] = hex_chars[i & 0xf];
            hex[i][2] = b' '; // space
        }
        Self {
            writer: std::io::BufWriter::new(std::io::stdout()),
            byte_to_color: ByteToColor::new(),
            hex,
            max_bytes_per_line,
            byte_counter: 0,
        }
    }

    // much faster version for this:
    // write!(&mut self.writer, "{:08x}", self.byte_counter)?;
    fn write_hex_byte_offset(&mut self) -> std::io::Result<()> {
        let mut hex_line_no: [u8; 8] = [0u8; 8];

        let mut bc = self.byte_counter;
        for i in 0..8 {
            hex_line_no[7 - i] = b"0123456789abcdef"[bc & 0xf];
            bc >>= 4;
        }

        self.writer.write_all(&hex_line_no)
    }

    #[inline(always)]
    fn write(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_all(text.as_bytes())
    }

    // output looks like this, for 'hexler /usr/bin/ls':
    //
    // /usr/bin/ls  146 KiB, 18 January 2024 00:00:00
    // ─────────┬───────────────────────────────────────────────────────────────────────────┬─────────────────────────
    // 00000000 │ 7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00  03 00 3e 00 01 00 00 00 │ ⌂ELF☻☺☺⋄⋄⋄⋄⋄⋄⋄⋄⋄♥⋄>⋄☺⋄⋄⋄
    // 00000018 │ 90 6e 00 00 00 00 00 00  40 00 00 00 00 00 00 00  e8 3f 02 00 00 00 00 00 │ Én⋄⋄⋄⋄⋄⋄@⋄⋄⋄⋄⋄⋄⋄Φ?☻⋄⋄⋄⋄⋄
    pub fn write_header(&mut self, title: &str) -> std::io::Result<()> {
        let num_groups = self.max_bytes_per_line / 8;
        let num_bytes_per_group = 8 * 3 + 1;

        self.write(title)?;
        self.write("\n")?;
        self.write("─────────┬")?;
        for _ in 0..(num_groups * num_bytes_per_group) {
            self.write("─")?;
        }
        self.write("┬")?;
        for _ in 0..(self.max_bytes_per_line + 1) {
            self.write("─")?;
        }
        self.write("\n")?;

        Ok(())
    }

    // Writes hex lines, like so:
    //
    // 00000000 │ 7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00  03 00 3e 00 01 00 00 00 │ ⌂ELF☻☺☺⋄⋄⋄⋄⋄⋄⋄⋄⋄♥⋄>⋄☺⋄⋄⋄
    pub fn write_line(&mut self, buffer: &[u8], bytes_in_buffer: usize) -> std::io::Result<()> {
        self.write_hex_byte_offset()?;
        self.write(" │")?;

        self.byte_counter += bytes_in_buffer;

        // write hex numbers "00 01 ..."
        let mut previous_color_id: u8 = 0;
        for i in 0..self.max_bytes_per_line {
            if i % 8 == 0 {
                self.writer.write_all(&Self::SPACE)?;
            }
            let hex = if i < bytes_in_buffer {
                &self.hex[buffer[i] as usize]
            } else {
                &Self::HEX_SPACE
            };
            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                let col = self.byte_to_color.color(buffer[i]);
                self.writer.write_all(&col.as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer.write_all(hex)?;
        }

        // Write codepage 437 characters
        if previous_color_id != 0 {
            self.writer.write_all(&Self::COLOR_RESET.as_bytes())?;
            previous_color_id = 0;
        }
        self.writer.write_all("│ ".as_bytes())?;
        // self.writer.write_all(&Self::SPACE)?;
        for i in 0..bytes_in_buffer {
            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                self.writer
                    .write_all(&self.byte_to_color.color(buffer[i]).as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer
                .write_all(Self::CODE_PAGE_437[buffer[i] as usize].as_bytes())?;
        }

        // done, newline!
        if previous_color_id != 0 {
            self.writer.write_all(&Self::COLOR_RESET.as_bytes())?;
        }
        self.writer.write_all(&Self::NEWLINE)?;
        Ok(())
    }

    // From the docs: It is critical to call flush before BufWriter<W> is dropped. Though dropping will attempt to flush the contents of
    // the buffer, any errors that happen in the process of dropping will be ignored. Calling flush ensures that the buffer is empty and
    // thus dropping will not even attempt file operations.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
