use std::collections::HashMap;

pub struct ByteToColor {
    color_ary: [&'static str; 256],
    color_id: [u8; 256],
}

impl Default for ByteToColor {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteToColor {
    const GREY: &'static str = "\x1b[90m"; // Bright black
                                           // const RED: &'static str = "\x1b[91m"; // Bright red
    const GREEN: &'static str = "\x1b[92m"; // Bright green
                                            // const YELLOW: &'static str = "\x1b[93m"; // Bright yellow
    const BLUE: &'static str = "\x1b[94m"; // Bright blue
    const MAGENTA: &'static str = "\x1b[95m"; // Bright magenta
                                              // const CYAN: &'static str = "\x1b[96m"; // Bright cyan
                                              // const WHITE: &'static str = "\x1b[97m"; // Bright white
    const RESET: &'static str = "\x1b[0m";
    //const BOLD: &'static str = "\x1b[1m";

    // This Rust function, new(), initializes an array of colors (colors) and associates each byte value with a
    // specific color. It creates the necessary mappings by iterating over all possible byte values from 0 to 255.
    // The function assigns colors based on certain criteria such as NUL or whitespace characters, symbols, digits,
    // uppercase letters, lowercase letters, and remaining high bytes.
    //
    // The color ID is stored in a separate array (color_id) for efficient lookup later
    pub fn new() -> Self {
        let mut colors = [Self::RESET; 256];
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
            colors[i as usize] = color;
            let val = color_to_id.entry(color).or_insert_with(|| {
                unique_color_count += 1;
                unique_color_count
            });
            color_id[i as usize] = *val;
        }

        Self {
            color_ary: colors,
            color_id,
        }
    }

    pub fn color(&self, byte: u8) -> &str {
        self.color_ary[byte as usize]
    }
    pub fn color_id(&self, byte: u8) -> u8 {
        self.color_id[byte as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nul_byte_color() {
        let btc = ByteToColor::new();
        assert_eq!(btc.color(0x00), ByteToColor::GREY);
    }

    #[test]
    fn test_del_byte_color() {
        let btc = ByteToColor::new();
        assert_eq!(btc.color(0x7f), ByteToColor::GREY);
    }

    #[test]
    fn test_extended_ascii_color() {
        let btc = ByteToColor::new();
        assert_eq!(btc.color(0xff), ByteToColor::GREY);
    }

    #[test]
    fn test_whitespace_colors() {
        let btc = ByteToColor::new();
        // LF, VT, FF, CR, SPACE
        assert_eq!(btc.color(0x0a), ByteToColor::GREEN); // LF
        assert_eq!(btc.color(0x0b), ByteToColor::GREEN); // VT
        assert_eq!(btc.color(0x0c), ByteToColor::GREEN); // FF
        assert_eq!(btc.color(0x0d), ByteToColor::GREEN); // CR
        assert_eq!(btc.color(0x20), ByteToColor::GREEN); // SPACE
    }

    #[test]
    fn test_digit_colors() {
        let btc = ByteToColor::new();
        for digit in b'0'..=b'9' {
            assert_eq!(btc.color(digit), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_uppercase_letter_colors() {
        let btc = ByteToColor::new();
        for letter in b'A'..=b'Z' {
            assert_eq!(btc.color(letter), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_lowercase_letter_colors() {
        let btc = ByteToColor::new();
        for letter in b'a'..=b'z' {
            assert_eq!(btc.color(letter), ByteToColor::RESET);
        }
    }

    #[test]
    fn test_symbol_colors() {
        let btc = ByteToColor::new();
        // Test various symbols
        assert_eq!(btc.color(b'!'), ByteToColor::MAGENTA);
        assert_eq!(btc.color(b'#'), ByteToColor::MAGENTA);
        assert_eq!(btc.color(b'@'), ByteToColor::MAGENTA);
        assert_eq!(btc.color(b'['), ByteToColor::MAGENTA);
        assert_eq!(btc.color(b'{'), ByteToColor::MAGENTA);
    }

    #[test]
    fn test_high_bytes_colors() {
        let btc = ByteToColor::new();
        for byte in 0x80..=0xfe {
            assert_eq!(btc.color(byte), ByteToColor::BLUE);
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
        assert_eq!(btc1.color(0x41), btc2.color(0x41));
        assert_eq!(btc1.color_id(0x41), btc2.color_id(0x41));
    }

    #[test]
    fn test_all_bytes_have_color() {
        let btc = ByteToColor::new();
        // Ensure every byte has a color assigned
        for byte in 0..=255u8 {
            let color = btc.color(byte);
            assert!(!color.is_empty(), "Byte {:02x} should have a color", byte);
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
