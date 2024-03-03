use std::io::prelude::*;

pub struct LineWriter {
    writer: std::io::BufWriter<std::io::Stdout>,
    max_bytes_per_line: usize,
    byte_counter: usize,
}

impl LineWriter {
    // https://de.wikipedia.org/wiki/Codepage_437
    #[rustfmt::skip]
    const CODE_PAGE_437: [&'static str; 256] = [
        "␀", "☺", "☻", "♥", "♦", "♣", "♠", "•", "◘", "○", "◙", "♂", "♀", "♪", "♫", "☼", // 00-0f
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
        "≡", "±", "≥", "≤", "⌠", "⌡", "÷", "≈", "°", "∙", "·", "√", "ⁿ", "²", "■", " ", // f0-ff
    ];

    #[rustfmt::skip]
    const HEX: [&'static str; 256] = [
        "00", "01", "02", "03", "04", "05", "06", "07",  "08", "09", "0a", "0b", "0c", "0d", "0e", "0f", // 00-0f
        "10", "11", "12", "13", "14", "15", "16", "17",  "18", "19", "1a", "1b", "1c", "1d", "1e", "1f", // 00-0f
        "20", "21", "22", "23", "24", "25", "26", "27",  "28", "29", "2a", "2b", "2c", "2d", "2e", "2f", // 00-0f
        "30", "31", "32", "33", "34", "35", "36", "37",  "38", "39", "3a", "3b", "3c", "3d", "3e", "3f", // 00-0f
        "40", "41", "42", "43", "44", "45", "46", "47",  "48", "49", "4a", "4b", "4c", "4d", "4e", "4f", // 00-0f
        "50", "51", "52", "53", "54", "55", "56", "57",  "58", "59", "5a", "5b", "5c", "5d", "5e", "5f", // 00-0f
        "60", "61", "62", "63", "64", "65", "66", "67",  "68", "69", "6a", "6b", "6c", "6d", "6e", "6f", // 00-0f
        "70", "71", "72", "73", "74", "75", "76", "77",  "78", "79", "7a", "7b", "7c", "7d", "7e", "7f", // 00-0f
        "80", "81", "82", "83", "84", "85", "86", "87",  "88", "89", "8a", "8b", "8c", "8d", "8e", "8f", // 00-0f
        "90", "91", "92", "93", "94", "95", "96", "97",  "98", "99", "9a", "9b", "9c", "9d", "9e", "9f", // 00-0f
        "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",  "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af", // 00-0f
        "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7",  "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf", // 00-0f
        "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7",  "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf", // 00-0f
        "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7",  "d8", "d9", "da", "db", "dc", "dd", "de", "df", // 00-0f
        "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7",  "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef", // 00-0f
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7",  "f8", "f9", "fa", "fb", "fc", "fd", "fe", "ff", // 00-0f
    ];

    const SPACE: &'static [u8] = b" ";
    const NEWLINE: &'static [u8] = b"\n";
    const HEX_SPACE: &'static [u8] = b"  ";

    pub fn new(max_bytes_per_line: usize) -> Self {
        Self {
            writer: std::io::BufWriter::new(std::io::stdout()),
            max_bytes_per_line,
            byte_counter: 0,
        }
    }

    pub fn write_line(&mut self, buffer: &[u8], bytes_in_buffer: usize) -> std::io::Result<()> {
        write!(&mut self.writer, "{:08x}: ", self.byte_counter)?;
        self.byte_counter += bytes_in_buffer;

        for i in 0..self.max_bytes_per_line {
            if i % 8 == 0 {
                self.writer.write_all(&Self::SPACE)?;
            }
            let hex = if i <= bytes_in_buffer {
                Self::HEX[buffer[i] as usize].as_bytes()
            } else {
                &Self::HEX_SPACE
            };
            self.writer.write_all(&hex)?;
            self.writer.write_all(&Self::SPACE)?;
        }
        self.writer.write_all(&Self::SPACE)?;
        for byte in &buffer[..bytes_in_buffer] {
            let bytes = Self::CODE_PAGE_437[*byte as usize].as_bytes();
            self.writer.write_all(&bytes).unwrap();
        }
        self.writer.write_all(&Self::NEWLINE)?;
        Ok(())
    }
}
