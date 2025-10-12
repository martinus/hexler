use crate::byte_to_color::ByteToColor;
use crate::error::{HexlerError, Result};

pub struct LineWriter<'a, T: 'a> {
    writer: &'a mut T,
    hex: [[u8; 3]; 256],
    byte_to_color: ByteToColor,
    bytes_per_line: usize,
    byte_counter: usize,
}

pub enum Border {
    Header,
    Footer,
}

impl<'a, T> LineWriter<'a, T>
where
    T: std::io::Write + 'a,
{
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
    const GREY: &'static str = "\x1b[90m";
    const COLOR_RESET: &'static str = "\x1b[0m";

    pub fn new_bytes(writer: &'a mut T, bytes_per_line: usize) -> Result<Self> {
        if bytes_per_line < 8 || 0 != bytes_per_line % 8 {
            Err(HexlerError::InvalidBytesPerLine(bytes_per_line))
        } else {
            let mut hex = [[0u8; 3]; 256];
            for i in 0..256 {
                let hex_chars = b"0123456789abcdef";
                hex[i][0] = hex_chars[i >> 4];
                hex[i][1] = hex_chars[i & 0xf];
                hex[i][2] = b' '; // space
            }
            Ok(Self {
                writer,
                byte_to_color: Default::default(),
                hex,
                bytes_per_line,
                byte_counter: 0,
            })
        }
    }

    // Calculates the maximum number of bytes allowed that fits into the given line width. Uses a minimum of 8 bytes.
    pub fn new_max_width(writer: &'a mut T, max_width: usize) -> Result<Self> {
        let mut num_groups_of_8: usize = 1;
        while 13 + (num_groups_of_8 + 1) * 33 <= max_width {
            num_groups_of_8 += 1;
        }

        Self::new_bytes(writer, num_groups_of_8 * 8)
    }

    pub fn bytes_per_line(&self) -> usize {
        self.bytes_per_line
    }

    // much faster version for this:
    // write!(&mut self.writer, "{:08x}", self.byte_counter)?;
    fn write_hex_byte_offset(&mut self) -> std::io::Result<()> {
        // only show a 32bit number. Ought to be large enough for everyone
        let bc = self.byte_counter as u32;
        let num_leading_hex_zeroes = bc.leading_zeros() / 4;
        self.write(Self::GREY)?;
        for _ in 0..num_leading_hex_zeroes {
            self.write("0")?;
        }
        self.write(Self::COLOR_RESET)?;

        // write the remaining hex digits
        for i in num_leading_hex_zeroes..8 {
            let n = bc >> (32 - i * 4 - 4);
            let c = b"0123456789abcdef"[n as usize & 0xf];
            self.writer.write_all(&[c])?;
        }

        Ok(())
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
    pub fn write_border(&mut self, border: Border, title: &str) -> std::io::Result<()> {
        let (connector, pre, post) = match border {
            Border::Header => ("┬", format!("{}\n", title), "\n".to_string()),
            Border::Footer => ("┴", "".to_string(), format!("{}\n", title)),
        };
        let num_groups = self.bytes_per_line / 8;
        let num_bytes_per_group = 8 * 3 + 1;

        self.write(pre.as_str())?;
        self.write("─────────")?;
        self.write(connector)?;
        for _ in 0..(num_groups * num_bytes_per_group) {
            self.write("─")?;
        }
        self.write(connector)?;
        for _ in 0..(self.bytes_per_line + 1) {
            self.write("─")?;
        }
        self.write(post.as_str())?;

        Ok(())
    }

    // Writes hex lines, like so:
    //
    // 00000000 │ 7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00  03 00 3e 00 01 00 00 00 │ ⌂ELF☻☺☺⋄⋄⋄⋄⋄⋄⋄⋄⋄♥⋄>⋄☺⋄⋄⋄
    #[allow(clippy::needless_range_loop)]
    pub fn write_line(&mut self, buffer: &[u8], bytes_in_buffer: usize) -> std::io::Result<()> {
        self.write_hex_byte_offset()?;
        self.write(" │")?;

        self.byte_counter += bytes_in_buffer;

        // write hex numbers "00 01 ..."
        let mut previous_color_id: u8 = 0;
        for i in 0..self.bytes_per_line {
            // Add an additional space after 8 bytes
            if i % 8 == 0 {
                self.writer.write_all(Self::SPACE)?;
            }
            let hex = if i < bytes_in_buffer {
                self.hex[buffer[i] as usize]
            } else {
                Self::HEX_SPACE
            };
            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                let col = self.byte_to_color.color(buffer[i]);
                self.writer.write_all(col.as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer.write_all(&hex)?;
        }

        // Write codepage 437 characters
        if previous_color_id != 0 {
            self.writer.write_all(Self::COLOR_RESET.as_bytes())?;
            previous_color_id = 0;
        }
        self.writer.write_all("│ ".as_bytes())?;
        // self.writer.write_all(&Self::SPACE)?;
        for i in 0..bytes_in_buffer {
            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                self.writer
                    .write_all(self.byte_to_color.color(buffer[i]).as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer
                .write_all(Self::CODE_PAGE_437[buffer[i] as usize].as_bytes())?;
        }

        // finished writing bytes, so reset color and finally go to the next line
        if previous_color_id != 0 {
            self.writer.write_all(Self::COLOR_RESET.as_bytes())?;
        }
        self.writer.write_all(Self::NEWLINE)?;
        Ok(())
    }

    // From the docs: It is critical to call flush before BufWriter<W> is dropped. Though dropping will attempt to flush the contents of
    // the buffer, any errors that happen in the process of dropping will be ignored. Calling flush ensures that the buffer is empty and
    // thus dropping will not even attempt file operations.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
