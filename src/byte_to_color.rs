use std::collections::HashMap;

pub struct ByteToColor {
    color_ary: [&'static str; 256],
    color_id: [u8; 256],
}

const ARRAY_REPEAT_VALUE: std::string::String = String::new();

impl ByteToColor {
    const BLACK: &'static str = "[30m";
    const RED: &'static str = "[31m";
    const GREEN: &'static str = "[32m";
    const YELLOW: &'static str = "[33m";
    const BLUE: &'static str = "[34m";
    const MAGENTA: &'static str = "[35m";
    const CYAN: &'static str = "[36m";
    const WHITE: &'static str = "[37m";
    const DEFAULT: &'static str = "[39m";
    const RESET: &'static str = "[0m";

    pub fn new() -> Self {
        let mut colors = [Self::DEFAULT; 256];
        let mut color_id = [0u8; 256];

        let mut unique_color_count = 0u8;
        let mut color_to_id = HashMap::<&str, u8>::new();

        for i in 0..=255u8 {
            let color = match i {
                // NUL, 0xff
                0x00 | 0x7f | 0xff => Self::BLACK,

                // whitespace
                0x0a | 0x0b | 0x0c | 0x0d | 0x20 => Self::GREEN,

                // symbols
                0x01..=0x1f | 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e => {
                    Self::MAGENTA
                }

                0x30..=0x39 => Self::DEFAULT, // digits
                0x41..=0x5a => Self::DEFAULT, // uppercase letters
                0x61..=0x7a => Self::DEFAULT, // lowercase letters

                // remaining high bytes
                0x80..=0xff => Self::BLUE,
            };
            colors[i as usize] = color;
            let val = color_to_id.entry(color).or_insert_with(|| {
                unique_color_count += 1;
                unique_color_count
            });
            color_id[i as usize] = *val;
        }

        // map each color to a unique
        // TODO: continue here
        for i in 0..256 {
            print!("{:02x}:{} ", i, color_id[i]);
        }

        Self {
            color_ary: colors,
            color_id: [0u8; 256],
        }
    }
}
