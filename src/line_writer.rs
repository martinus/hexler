use crate::ascii_renderer::AsciiRenderer;
use crate::border_writer::BorderWriter;
use crate::byte_to_color::ByteToColor;
use crate::error::{HexlerError, Result};
use crate::hex_formatter::HexFormatter;

/// Orchestrates the hex dump line output by coordinating specialized components.
///
/// This is the main component that brings together:
/// - `HexFormatter`: Converts bytes to hexadecimal representation
/// - `AsciiRenderer`: Renders bytes as CodePage 437 characters
/// - `ByteToColor`: Assigns ANSI colors to bytes based on their type
/// - `BorderWriter`: Draws Unicode borders for visual separation
///
/// Output format: `00000000 │ 7f 45 4c 46 ... │ ⌂ELF...`
pub struct LineWriter {
    hex_formatter: HexFormatter,
    ascii_renderer: AsciiRenderer,
    byte_to_color: ByteToColor,
    bytes_per_line: usize,
    byte_counter: usize,
}

/// Border type for headers and footers.
pub enum Border {
    Header,
    Footer,
}

impl LineWriter {
    const COLOR_RESET: &'static [u8] = b"\x1b[0m";

    /// Creates a new LineWriter with a specified number of bytes per line.
    ///
    /// # Arguments
    /// * `bytes_per_line` - Must be a multiple of 8 (for proper alignment) and at least 8
    ///
    /// # Errors
    /// Returns `InvalidBytesPerLine` if the value doesn't meet the requirements.
    pub fn new_bytes(bytes_per_line: usize) -> Result<Self> {
        if bytes_per_line < 8 || bytes_per_line & 7 != 0 {
            // Faster than is_multiple_of(8)
            Err(HexlerError::InvalidBytesPerLine(bytes_per_line))
        } else {
            Ok(Self {
                hex_formatter: HexFormatter::new(),
                ascii_renderer: AsciiRenderer::new(),
                byte_to_color: ByteToColor::new(),
                bytes_per_line,
                byte_counter: 0,
            })
        }
    }

    /// Creates a new LineWriter with the maximum bytes_per_line that fits within the given width.
    ///
    /// Calculates the optimal number of byte groups (8 bytes each) that fit within the
    /// specified character width. Uses a minimum of 8 bytes (1 group).
    ///
    /// # Arguments
    /// * `max_width` - Maximum line width in characters
    pub fn new_max_width(max_width: usize) -> Result<Self> {
        let mut num_groups_of_8: usize = 1;
        while 13 + (num_groups_of_8 + 1) * 33 <= max_width {
            num_groups_of_8 += 1;
        }

        Self::new_bytes(num_groups_of_8 * 8)
    }

    /// Returns the number of bytes displayed per line.
    pub fn bytes_per_line(&self) -> usize {
        self.bytes_per_line
    }

    /// Writes a header or footer border with an optional title to the provided buffer.
    pub fn write_border(&mut self, buffer: &mut Vec<u8>, border: Border, title: &str) -> std::io::Result<()> {
        match border {
            Border::Header => BorderWriter::write_header(buffer, title, self.bytes_per_line),
            Border::Footer => BorderWriter::write_footer(buffer, title, self.bytes_per_line),
        }
    }

    /// Writes a complete hex dump line with offset, hex bytes, and ASCII representation.
    ///
    /// Format: `00000000 │ 7f 45 4c 46 ... │ ⌂ELF...`
    ///
    /// The line has three sections:
    /// 1. Offset (8 hex digits with grey leading zeros)
    /// 2. Hex bytes (color-coded, grouped by 8)
    /// 3. ASCII (CodePage 437 characters, color-coded)
    ///
    /// Colors are only written when they change to minimize output size.
    ///
    /// # Arguments
    /// * `buffer` - Output buffer to append the line to (not cleared, only appended)
    /// * `line_data` - Byte buffer to display (must be at least `bytes_in_buffer` long)
    /// * `bytes_in_buffer` - Number of valid bytes in the buffer (may be less than bytes_per_line for the last line)
    pub fn write_line(&mut self, buffer: &mut Vec<u8>, line_data: &[u8], bytes_in_buffer: usize) {
        // Write hex offset
        self.hex_formatter
            .write_offset(buffer, self.byte_counter);
        buffer.extend_from_slice(b" \xE2\x94\x82"); // " │" in UTF-8

        self.byte_counter += bytes_in_buffer;

        // Write hex numbers "00 01 ..."
        let mut previous_color_id: u8 = 0;

        // Process actual bytes
        let mut group_counter = 0;
        for &byte in &line_data[..bytes_in_buffer] {
            // Add an additional space after 8 bytes
            if group_counter == 0 {
                buffer.push(b' ');
            }
            group_counter = (group_counter + 1) & 7; // Faster than %8 or is_multiple_of(8)

            let next_color_id = self.byte_to_color.id(byte);
            if next_color_id != previous_color_id {
                buffer
                    .extend_from_slice(self.byte_to_color.bytes(byte));
                previous_color_id = next_color_id;
            }
            buffer
                .extend_from_slice(self.hex_formatter.hex_byte(byte));
        }

        // Fill remaining space with padding
        let padding_count = self.bytes_per_line - bytes_in_buffer;

        // Calculate number of separator spaces: count how many times group_counter wraps to 0
        let num_separators =
            ((bytes_in_buffer & 7) + padding_count) / 8 - (padding_count % 8 != 0) as usize;
        let padding_size = num_separators + padding_count * 3;
        buffer
            .resize(buffer.len() + padding_size, b' ');

        // Write codepage 437 characters
        if previous_color_id != 0 {
            buffer.extend_from_slice(Self::COLOR_RESET);
            previous_color_id = 0;
        }
        buffer.extend_from_slice(b"\xE2\x94\x82 "); // "│ " in UTF-8

        for &byte in &line_data[..bytes_in_buffer] {
            let next_color_id = self.byte_to_color.id(byte);
            if next_color_id != previous_color_id {
                buffer
                    .extend_from_slice(self.byte_to_color.bytes(byte));
                previous_color_id = next_color_id;
            }
            buffer
                .extend_from_slice(self.ascii_renderer.render_bytes(byte));
        }

        // Finished writing bytes, so reset color and finally go to the next line
        if previous_color_id != 0 {
            buffer.extend_from_slice(Self::COLOR_RESET);
        }
        buffer.push(b'\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bytes_valid() {
        let result = LineWriter::new_bytes(8);
        assert!(result.is_ok());

        let result = LineWriter::new_bytes(16);
        assert!(result.is_ok());

        let result = LineWriter::new_bytes(32);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_bytes_invalid_too_small() {
        let result = LineWriter::new_bytes(4);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_bytes_invalid_not_multiple() {
        let result = LineWriter::new_bytes(12);
        assert!(result.is_err());

        let result = LineWriter::new_bytes(20);
        assert!(result.is_err());
    }

    #[test]
    fn test_bytes_per_line() {
        let line_writer = LineWriter::new_bytes(24).unwrap();
        assert_eq!(line_writer.bytes_per_line(), 24);
    }

    #[test]
    fn test_write_border_header() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();
        line_writer
            .write_border(&mut buffer, Border::Header, "Test Header")
            .unwrap();

        let output = String::from_utf8_lossy(&buffer);
        assert!(output.contains("Test Header"));
        assert!(output.contains("─")); // horizontal border
        assert!(output.contains("┬")); // top connector
    }

    #[test]
    fn test_write_border_footer() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();
        line_writer
            .write_border(&mut buffer, Border::Footer, "Test Footer")
            .unwrap();

        let output = String::from_utf8_lossy(&buffer);
        assert!(output.contains("Test Footer"));
        assert!(output.contains("─")); // horizontal border
        assert!(output.contains("┴")); // bottom connector
    }

    #[test]
    fn test_write_line_full() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let line_data = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x00, 0xff]; // "Hello!" + NUL + 0xff
        line_writer.write_line(&mut buffer, &line_data, 8);

        let output = String::from_utf8_lossy(&buffer);
        // Hex values might have ANSI color codes between them
        assert!(output.contains("48"));
        assert!(output.contains("65"));
        assert!(output.contains("6c"));
        assert!(output.contains("6f"));
        assert!(output.contains("21"));
        assert!(output.contains("│")); // separator
    }

    #[test]
    fn test_write_line_partial() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(16).unwrap();

        let mut line_data = [0u8; 16];
        line_data[0] = 0x41; // 'A'
        line_data[1] = 0x42; // 'B'
        line_data[2] = 0x43; // 'C'

        line_writer.write_line(&mut buffer, &line_data, 3);

        let output = String::from_utf8_lossy(&buffer);
        assert!(output.contains("41 42 43")); // The 3 bytes we wrote
                                              // Should have spacing for remaining bytes
    }

    #[test]
    fn test_write_line_increments_byte_counter() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let line_data = [0u8; 8];
        line_writer.write_line(&mut buffer, &line_data, 8);

        // Write another line - offset should have changed
        line_writer.write_line(&mut buffer, &line_data, 8);
        let output = String::from_utf8_lossy(&buffer);

        // Should show first offset all zeros
        assert!(output.contains("00000000"));
        // The output has ANSI codes, so just check offset changes
        // by checking that we have a second line with different offset chars
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() >= 2, "Should have at least 2 lines");
    }

    #[test]
    fn test_codepage_437_characters() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let line_data = [0x00, 0x01, 0x02, 0x20, 0x41, 0x80, 0x81, 0xff];
        line_writer.write_line(&mut buffer, &line_data, 8);

        let output = String::from_utf8_lossy(&buffer);
        // Should contain codepage 437 representations
        assert!(output.contains("⋄")); // 0x00
        assert!(output.contains("☺")); // 0x01
        assert!(output.contains(" ")); // 0x20
        assert!(output.contains("A")); // 0x41
    }

    #[test]
    fn test_new_max_width_small() {
        let line_writer = LineWriter::new_max_width(50).unwrap();
        // Should default to minimum (8 bytes)
        assert_eq!(line_writer.bytes_per_line(), 8);
    }

    #[test]
    fn test_new_max_width_large() {
        let line_writer = LineWriter::new_max_width(200).unwrap();
        // Should allow more than minimum bytes
        assert!(line_writer.bytes_per_line() > 8);
        // Should be multiple of 8
        assert_eq!(line_writer.bytes_per_line() % 8, 0);
    }

    #[test]
    fn test_hex_offset_leading_zeros() {
        let mut buffer = Vec::new();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let line_data = [0u8; 8];
        line_writer.write_line(&mut buffer, &line_data, 8);

        let output = String::from_utf8_lossy(&buffer);
        // First line should have leading zeros
        assert!(output.contains("00000000"));
    }
}
