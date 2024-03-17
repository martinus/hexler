use std::collections::HashMap;

pub struct ByteToColor {
    color_ary: [&'static str; 256],
    color_id: [u8; 256],
}

impl ByteToColor {
    const GREY: &'static str = "[90m";
    // const RED: &'static str = "[31m";
    const GREEN: &'static str = "[32m";
    // const YELLOW: &'static str = "[33m";
    const BLUE: &'static str = "[34m";
    const MAGENTA: &'static str = "[35m";
    // const CYAN: &'static str = "[36m";
    // const WHITE: &'static str = "[37m";
    const RESET: &'static str = "[0m";
    //const BOLD: &'static str = "[1m";

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
                // NUL, 0xff
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
                0x80..=0xff => Self::BLUE,
            };
            colors[i as usize] = color;
            let val = color_to_id.entry(color).or_insert_with(|| {
                unique_color_count += 1;
                unique_color_count
            });
            color_id[i as usize] = *val;

            /*
            if i % 32 == 0 {
                print!("\n");
            }
            print!("{}", *val);
            */
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
