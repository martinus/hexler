use std::collections::HashMap;

pub struct ByteToColor {
    color_id: [u8; 256],
    color_bytes: [&'static [u8]; 256], // Pre-computed as_bytes() for each color
}

impl Default for ByteToColor {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteToColor {
    // I'm using 256-color ANSI escape codes here because the terminal in VSCode has a bug with the normal colors:
    // it doesn't use the correct brightness when the following character is a UTF-8 character.

    const GREY: &'static str = "\x1b[38;5;8m"; // 256-color grey for special bytes
    const GREEN: &'static str = "\x1b[38;5;46m"; // 256-color bright green
    const BLUE: &'static str = "\x1b[38;5;33m"; // 256-color bright blue
    const MAGENTA: &'static str = "\x1b[38;5;201m"; // 256-color bright magenta
    const RESET: &'static str = "\x1b[0m";

    /// Creates a new ByteToColor instance with color mappings for all 256 byte values.
    ///
    /// Color assignments:
    /// - NUL (0x00), DEL (0x7F), and 0xFF: Grey (special control bytes)
    /// - Whitespace (LF, VT, FF, CR, SPACE): Green
    /// - Symbols and control characters: Magenta
    /// - Digits, letters: No color (reset)
    /// - High bytes (0x80-0xFE): Blue (extended ASCII/CodePage 437)
    ///
    /// The color_id array stores unique identifiers for each color to enable
    /// efficient color change detection when rendering.
    pub fn new() -> Self {
        let mut color_bytes = [Self::RESET.as_bytes(); 256];
        let mut color_id = [0u8; 256];

        let mut unique_color_count = 0u8;
        let mut color_to_id = HashMap::<&str, u8>::new();

        // id 0 is reset!
        color_to_id.insert(Self::RESET, 0);

        for i in 0..=255u8 {
            let color = match i {
                // NUL, DEL, 0xff
                0x00 | 0x7f | 0xff => Self::GREY,

                // whitespace
                0x0a | 0x0b | 0x0c | 0x0d | 0x20 => Self::GREEN,

                // symbols
                0x01..=0x1f | 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e => {
                    Self::MAGENTA
                }

                0x30..=0x39 => Self::RESET, // digits
                0x41..=0x5a => Self::RESET, // uppercase letters
                0x61..=0x7a => Self::RESET, // lowercase letters

                // remaining high bytes
                0x80..=0xfe => Self::BLUE,
            };
            color_bytes[i as usize] = color.as_bytes();
            let val = color_to_id.entry(color).or_insert_with(|| {
                unique_color_count += 1;
                unique_color_count
            });
            color_id[i as usize] = *val;
        }

        Self {
            color_id,
            color_bytes,
        }
    }

    /// Returns the ANSI color escape code as bytes for the given byte.
    /// This is more efficient than calling color().as_bytes() in hot loops.
    #[inline]
    pub fn color_bytes(&self, byte: u8) -> &'static [u8] {
        self.color_bytes[byte as usize]
    }

    /// Returns the color ID for the given byte.
    ///
    /// Color IDs are used to detect when the color changes between consecutive bytes,
    /// allowing us to output ANSI color codes only when necessary.
    pub fn color_id(&self, byte: u8) -> u8 {
        self.color_id[byte as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for tests to get color as &str
    fn color(btc: &ByteToColor, byte: u8) -> &str {
        // Safe because color_bytes always contains valid UTF-8 ANSI escape codes
        unsafe { std::str::from_utf8_unchecked(btc.color_bytes(byte)) }
    }

    #[test]
    fn test_nul_byte_color() {
        let btc = ByteToColor::new();
        assert_eq!(color(&btc, 0x00), ByteToColor::GREY);
    }

    #[test]
    fn test_del_byte_color() {
        let btc = ByteToColor::new();
        assert_eq!(color(&btc, 0x7f), ByteToColor::GREY);
    }

    #[test]
    fn test_extended_ascii_color() {
        let btc = ByteToColor::new();
        assert_eq!(color(&btc, 0xff), ByteToColor::GREY);
    }

    #[test]
    fn test_whitespace_colors() {
        let btc = ByteToColor::new();
        // LF, VT, FF, CR, SPACE
        assert_eq!(color(&btc, 0x0a), ByteToColor::GREEN); // LF
        assert_eq!(color(&btc, 0x0b), ByteToColor::GREEN); // VT
        assert_eq!(color(&btc, 0x0c), ByteToColor::GREEN); // FF
        assert_eq!(color(&btc, 0x0d), ByteToColor::GREEN); // CR
        assert_eq!(color(&btc, 0x20), ByteToColor::GREEN); // SPACE
    }

    #[test]
    fn test_digit_colors() {
        let btc = ByteToColor::new();
        for digit in b'0'..=b'9' {
            assert_eq!(color(&btc, digit), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_uppercase_letter_colors() {
        let btc = ByteToColor::new();
        for letter in b'A'..=b'Z' {
            assert_eq!(color(&btc, letter), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_lowercase_letter_colors() {
        let btc = ByteToColor::new();
        for letter in b'a'..=b'z' {
            assert_eq!(color(&btc, letter), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_symbol_colors() {
        let btc = ByteToColor::new();
        // Test various symbols
        assert_eq!(color(&btc, b'!'), ByteToColor::MAGENTA);
        assert_eq!(color(&btc, b'#'), ByteToColor::MAGENTA);
        assert_eq!(color(&btc, b'@'), ByteToColor::MAGENTA);
        assert_eq!(color(&btc, b'['), ByteToColor::MAGENTA);
        assert_eq!(color(&btc, b'{'), ByteToColor::MAGENTA);
    }

    #[test]
    fn test_high_bytes_colors() {
        let btc = ByteToColor::new();
        for byte in 0x80..=0xfe {
            assert_eq!(color(&btc, byte), ByteToColor::BLUE);
        }
    }

    #[test]
    fn test_color_id_consistency() {
        let btc = ByteToColor::new();
        // Same color should have same ID
        let id1 = btc.color_id(b'A');
        let id2 = btc.color_id(b'B');
        assert_eq!(id1, id2, "Uppercase letters should have the same color ID");

        // Different colors should have different IDs
        let id_grey = btc.color_id(0x00);
        let id_green = btc.color_id(0x20);
        assert_ne!(
            id_grey, id_green,
            "Different colors should have different IDs"
        );
    }

    #[test]
    fn test_default_trait() {
        let btc1 = ByteToColor::new();
        let btc2 = ByteToColor::default();

        // Both should produce same colors
        assert_eq!(color(&btc1, 0x41), color(&btc2, 0x41));
        assert_eq!(btc1.color_id(0x41), btc2.color_id(0x41));
    }

    #[test]
    fn test_all_bytes_have_color() {
        let btc = ByteToColor::new();
        // Ensure every byte has a color assigned
        for byte in 0..=255u8 {
            let c = color(&btc, byte);
            assert!(!c.is_empty(), "Byte {:02x} should have a color", byte);
        }
    }

    #[test]
    fn test_color_id_is_valid() {
        let btc = ByteToColor::new();
        // Color ID 0 should be RESET
        for byte in 0..=255u8 {
            let color_id = btc.color_id(byte);
            // ID should be reasonable (not too many unique colors)
            assert!(
                color_id < 10,
                "Color ID should be small, got {} for byte {:02x}",
                color_id,
                byte
            );
        }
    }
}
