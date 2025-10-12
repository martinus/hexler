/// Handles hexadecimal formatting for bytes and file offsets.
///
/// Uses a pre-computed lookup table for fast hex conversion of all 256 possible byte values.
/// Each byte is formatted as two hex digits plus a trailing space (e.g., "ff ").
pub struct HexFormatter {
    hex_lookup: [[u8; 3]; 256],
}

impl HexFormatter {
    const GREY: &'static str = "\x1b[90m";
    const COLOR_RESET: &'static str = "\x1b[0m";
    const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

    /// Creates a new HexFormatter with pre-computed hex lookup table.
    ///
    /// The lookup table contains all 256 byte values as "XX " (two hex digits + space),
    /// enabling O(1) hex conversion without runtime computation.
    pub fn new() -> Self {
        let mut hex_lookup = [[0u8; 3]; 256];
        for i in 0..256 {
            hex_lookup[i][0] = Self::HEX_CHARS[i >> 4];
            hex_lookup[i][1] = Self::HEX_CHARS[i & 0xf];
            hex_lookup[i][2] = b' '; // space
        }
        Self { hex_lookup }
    }

    /// Returns the hex representation of a byte as a 3-byte array: "XX ".
    ///
    /// # Example
    /// ```
    /// use hexler::hex_formatter::HexFormatter;
    /// let formatter = HexFormatter::new();
    /// assert_eq!(formatter.hex_byte(0xff), b"ff ");
    /// assert_eq!(formatter.hex_byte(0x00), b"00 ");
    /// ```
    pub fn hex_byte(&self, byte: u8) -> &[u8; 3] {
        &self.hex_lookup[byte as usize]
    }

    /// Returns three spaces "   " used for padding when a byte position is empty.
    pub fn hex_space() -> &'static [u8; 3] {
        &[b' ', b' ', b' ']
    }

    /// Writes a byte offset as 8 hex digits with leading zeros displayed in grey.
    ///
    /// Leading zeros are rendered in grey color to improve readability by
    /// de-emphasizing them. The significant digits remain in the default color.
    ///
    /// # Examples
    /// - `0x00000000` → grey "00000000"
    /// - `0x00001234` → grey "0000" + "1234"
    /// - `0x12345678` → "12345678" (no leading zeros)
    pub fn write_offset<W: std::io::Write>(
        &self,
        writer: &mut W,
        offset: usize,
    ) -> std::io::Result<()> {
        let bc = offset as u32;
        let num_leading_hex_zeroes = bc.leading_zeros() / 4;
        
        // Write leading zeros in grey
        writer.write_all(Self::GREY.as_bytes())?;
        for _ in 0..num_leading_hex_zeroes {
            writer.write_all(b"0")?;
        }
        writer.write_all(Self::COLOR_RESET.as_bytes())?;

        // Write the remaining hex digits
        for i in num_leading_hex_zeroes..8 {
            let n = bc >> (32 - i * 4 - 4);
            let c = Self::HEX_CHARS[n as usize & 0xf];
            writer.write_all(&[c])?;
        }

        Ok(())
    }
}

impl Default for HexFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_byte() {
        let formatter = HexFormatter::new();
        assert_eq!(formatter.hex_byte(0x00), b"00 ");
        assert_eq!(formatter.hex_byte(0xff), b"ff ");
        assert_eq!(formatter.hex_byte(0x41), b"41 "); // 'A'
    }

    #[test]
    fn test_hex_space() {
        assert_eq!(HexFormatter::hex_space(), b"   ");
    }

    #[test]
    fn test_write_offset() {
        let formatter = HexFormatter::new();
        let mut output = Vec::new();
        
        formatter.write_offset(&mut output, 0).unwrap();
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("00000000"));
        
        output.clear();
        formatter.write_offset(&mut output, 0x1234).unwrap();
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("1234"));
    }

    #[test]
    fn test_all_bytes() {
        let formatter = HexFormatter::new();
        for i in 0..=255u8 {
            let hex = formatter.hex_byte(i);
            assert_eq!(hex.len(), 3);
            assert_eq!(hex[2], b' '); // trailing space
        }
    }
}
