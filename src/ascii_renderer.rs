/// Handles rendering bytes as CodePage 437 characters.
///
/// CodePage 437 is the character encoding used by the original IBM PC.
/// It maps all 256 byte values to printable characters, including special
/// symbols for control characters (0x00-0x1F) and extended ASCII (0x80-0xFF).
pub struct AsciiRenderer {
    codepage_437: [&'static str; 256],
}

impl AsciiRenderer {
    /// Creates a new AsciiRenderer with the full CodePage 437 character set.
    pub fn new() -> Self {
        #[rustfmt::skip]
        let codepage_437 = [
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

        Self { codepage_437 }
    }

    /// Returns the CodePage 437 character representation of a byte.
    ///
    /// # Examples
    /// - `0x00` (NUL) → "⋄"
    /// - `0x41` (ASCII 'A') → "A"
    /// - `0x80` → "Ç"
    /// - `0xFF` → "ﬀ"
    pub fn render(&self, byte: u8) -> &str {
        self.codepage_437[byte as usize]
    }
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_characters() {
        let renderer = AsciiRenderer::new();
        assert_eq!(renderer.render(0x00), "⋄"); // NUL
        assert_eq!(renderer.render(0x01), "☺"); // SOH
        assert_eq!(renderer.render(0x02), "☻"); // STX
    }

    #[test]
    fn test_printable_ascii() {
        let renderer = AsciiRenderer::new();
        assert_eq!(renderer.render(0x20), " "); // space
        assert_eq!(renderer.render(0x41), "A");
        assert_eq!(renderer.render(0x61), "a");
        assert_eq!(renderer.render(0x30), "0");
    }

    #[test]
    fn test_extended_ascii() {
        let renderer = AsciiRenderer::new();
        assert_eq!(renderer.render(0x80), "Ç");
        assert_eq!(renderer.render(0xff), "ﬀ");
    }

    #[test]
    fn test_all_bytes_have_representation() {
        let renderer = AsciiRenderer::new();
        for byte in 0..=255u8 {
            let char = renderer.render(byte);
            assert!(
                !char.is_empty(),
                "Byte {:02x} should have a representation",
                byte
            );
        }
    }

    #[test]
    fn test_default_trait() {
        let renderer1 = AsciiRenderer::new();
        let renderer2 = AsciiRenderer::default();
        assert_eq!(renderer1.render(0x41), renderer2.render(0x41));
    }
}
