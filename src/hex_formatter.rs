/// Handles hexadecimal formatting for bytes and file offsets.
///
/// Uses a pre-computed lookup table for fast hex conversion of all 256 possible byte values.
/// Each byte is formatted as two hex digits plus a trailing space (e.g., "ff ").
pub struct HexFormatter {
    hex_lookup: [[u8; 3]; 256],
}

impl HexFormatter {
    const GREY: &'static [u8] = b"\x1b[90m";
    const COLOR_RESET: &'static [u8] = b"\x1b[0m";
    const HEX_CHARS: &'static [u8] = b"0123456789abcdef";

    /// Creates a new HexFormatter with pre-computed hex lookup table.
    ///
    /// The lookup table contains all 256 byte values as "XX " (two hex digits + space),
    /// enabling O(1) hex conversion without runtime computation.
    pub fn new() -> Self {
        let mut hex_lookup = [[0u8; 3]; 256];
        for (i, item) in hex_lookup.iter_mut().enumerate() {
            item[0] = Self::HEX_CHARS[i >> 4];
            item[1] = Self::HEX_CHARS[i & 0xf];
            item[2] = b' '; // space
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
        b"   "
    }

    /// Writes a byte offset directly into a `Vec<u8>` buffer.
    ///
    /// Optimized version that appends directly to a Vec without going through Write trait.
    ///
    /// # Examples
    /// - `0x00000000` → grey "00000000"
    /// - `0x00001234` → grey "0000" + "1234"
    /// - `0x12345678` → "12345678" (no leading zeros)
    pub fn write_offset(&self, buf: &mut Vec<u8>, offset: usize) {
        let bc = offset as u32;
        let num_leading_hex_zeroes = bc.leading_zeros() / 4;

        // Append grey color code
        buf.extend_from_slice(Self::GREY);

        // Append leading zeros efficiently
        let start_len = buf.len();
        buf.resize(start_len + num_leading_hex_zeroes as usize, b'0');

        // Append color reset
        buf.extend_from_slice(Self::COLOR_RESET);

        // Append the remaining hex digits
        for i in num_leading_hex_zeroes..8 {
            let n = bc >> (32 - i * 4 - 4);
            buf.push(Self::HEX_CHARS[n as usize & 0xf]);
        }
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

        // Test offset 0 - should have all leading zeros in grey
        formatter.write_offset(&mut output, 0);
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("00000000"));
        assert!(result.contains("\x1b[90m")); // Grey color code
        assert!(result.contains("\x1b[0m")); // Reset code

        // Test offset with some leading zeros
        output.clear();
        formatter.write_offset(&mut output, 0x1234);
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("1234"));
        // Should have grey section with "0000", then reset, then "1234"
        assert!(result.ends_with("1234"));

        // Test offset with no leading zeros (all 8 digits are significant)
        output.clear();
        formatter.write_offset(&mut output, 0x12345678);
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("12345678"));

        // Test that max offset works
        output.clear();
        formatter.write_offset(&mut output, 0xFFFFFFFF);
        let result = String::from_utf8_lossy(&output);
        assert!(result.ends_with("ffffffff"));

        // Test offset with single leading zero
        output.clear();
        formatter.write_offset(&mut output, 0x0FFFFFFF);
        let result = String::from_utf8_lossy(&output);
        assert!(result.ends_with("fffffff"));
        assert!(result.contains("\x1b[90m0\x1b[0m")); // One grey zero
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
