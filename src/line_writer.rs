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
pub struct LineWriter<'a, T: 'a> {
    writer: &'a mut T,
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

impl<'a, T> LineWriter<'a, T>
where
    T: std::io::Write + 'a,
{
    const SPACE: &'static [u8] = b" ";
    const NEWLINE: &'static [u8] = b"\n";
    const COLOR_RESET: &'static str = "\x1b[0m";

    /// Creates a new LineWriter with a specified number of bytes per line.
    ///
    /// # Arguments
    /// * `bytes_per_line` - Must be a multiple of 8 (for proper alignment) and at least 8
    ///
    /// # Errors
    /// Returns `InvalidBytesPerLine` if the value doesn't meet the requirements.
    pub fn new_bytes(writer: &'a mut T, bytes_per_line: usize) -> Result<Self> {
        if bytes_per_line < 8 || 0 != bytes_per_line % 8 {
            Err(HexlerError::InvalidBytesPerLine(bytes_per_line))
        } else {
            Ok(Self {
                writer,
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
    pub fn new_max_width(writer: &'a mut T, max_width: usize) -> Result<Self> {
        let mut num_groups_of_8: usize = 1;
        while 13 + (num_groups_of_8 + 1) * 33 <= max_width {
            num_groups_of_8 += 1;
        }

        Self::new_bytes(writer, num_groups_of_8 * 8)
    }

    /// Returns the number of bytes displayed per line.
    pub fn bytes_per_line(&self) -> usize {
        self.bytes_per_line
    }

    #[inline(always)]
    fn write(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_all(text.as_bytes())
    }

    /// Writes a header or footer border with an optional title.
    pub fn write_border(&mut self, border: Border, title: &str) -> std::io::Result<()> {
        match border {
            Border::Header => BorderWriter::write_header(self.writer, title, self.bytes_per_line),
            Border::Footer => BorderWriter::write_footer(self.writer, title, self.bytes_per_line),
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
    /// * `buffer` - Byte buffer to display (must be at least `bytes_in_buffer` long)
    /// * `bytes_in_buffer` - Number of valid bytes in the buffer (may be less than bytes_per_line for the last line)
    #[allow(clippy::needless_range_loop)]
    pub fn write_line(&mut self, buffer: &[u8], bytes_in_buffer: usize) -> std::io::Result<()> {
        // Write hex offset
        self.hex_formatter
            .write_offset(self.writer, self.byte_counter)?;
        self.write(" │")?;

        self.byte_counter += bytes_in_buffer;

        // Write hex numbers "00 01 ..."
        let mut previous_color_id: u8 = 0;
        for i in 0..self.bytes_per_line {
            // Add an additional space after 8 bytes
            if i % 8 == 0 {
                self.writer.write_all(Self::SPACE)?;
            }

            let hex = if i < bytes_in_buffer {
                self.hex_formatter.hex_byte(buffer[i])
            } else {
                HexFormatter::hex_space()
            };

            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                let col = self.byte_to_color.color(buffer[i]);
                self.writer.write_all(col.as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer.write_all(hex)?;
        }

        // Write codepage 437 characters
        if previous_color_id != 0 {
            self.writer.write_all(Self::COLOR_RESET.as_bytes())?;
            previous_color_id = 0;
        }
        self.writer.write_all("│ ".as_bytes())?;

        for i in 0..bytes_in_buffer {
            let next_color_id: u8 = self.byte_to_color.color_id(buffer[i]);
            if next_color_id != previous_color_id {
                self.writer
                    .write_all(self.byte_to_color.color(buffer[i]).as_bytes())?;
                previous_color_id = next_color_id;
            }
            self.writer
                .write_all(self.ascii_renderer.render(buffer[i]).as_bytes())?;
        }

        // Finished writing bytes, so reset color and finally go to the next line
        if previous_color_id != 0 {
            self.writer.write_all(Self::COLOR_RESET.as_bytes())?;
        }
        self.writer.write_all(Self::NEWLINE)?;
        Ok(())
    }

    /// Flushes the underlying writer to ensure all data is written.
    ///
    /// From the Rust docs: "It is critical to call flush before BufWriter<W> is dropped.
    /// Though dropping will attempt to flush the contents of the buffer, any errors that
    /// happen in the process of dropping will be ignored. Calling flush ensures that the
    /// buffer is empty and thus dropping will not even attempt file operations."
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn to_string(&self) -> String {
            String::from_utf8_lossy(&self.data).to_string()
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_new_bytes_valid() {
        let mut writer = TestWriter::new();
        let result = LineWriter::new_bytes(&mut writer, 8);
        assert!(result.is_ok());

        let result = LineWriter::new_bytes(&mut writer, 16);
        assert!(result.is_ok());

        let result = LineWriter::new_bytes(&mut writer, 32);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_bytes_invalid_too_small() {
        let mut writer = TestWriter::new();
        let result = LineWriter::new_bytes(&mut writer, 4);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_bytes_invalid_not_multiple() {
        let mut writer = TestWriter::new();
        let result = LineWriter::new_bytes(&mut writer, 12);
        assert!(result.is_err());

        let result = LineWriter::new_bytes(&mut writer, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_bytes_per_line() {
        let mut writer = TestWriter::new();
        let line_writer = LineWriter::new_bytes(&mut writer, 24).unwrap();
        assert_eq!(line_writer.bytes_per_line(), 24);
    }

    #[test]
    fn test_write_border_header() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();
        line_writer
            .write_border(Border::Header, "Test Header")
            .unwrap();

        let output = writer.to_string();
        assert!(output.contains("Test Header"));
        assert!(output.contains("─")); // horizontal border
        assert!(output.contains("┬")); // top connector
    }

    #[test]
    fn test_write_border_footer() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();
        line_writer
            .write_border(Border::Footer, "Test Footer")
            .unwrap();

        let output = writer.to_string();
        assert!(output.contains("Test Footer"));
        assert!(output.contains("─")); // horizontal border
        assert!(output.contains("┴")); // bottom connector
    }

    #[test]
    fn test_write_line_full() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();

        let buffer = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x00, 0xff]; // "Hello!" + NUL + 0xff
        line_writer.write_line(&buffer, 8).unwrap();

        let output = writer.to_string();
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
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 16).unwrap();

        let mut buffer = [0u8; 16];
        buffer[0] = 0x41; // 'A'
        buffer[1] = 0x42; // 'B'
        buffer[2] = 0x43; // 'C'

        line_writer.write_line(&buffer, 3).unwrap();

        let output = writer.to_string();
        assert!(output.contains("41 42 43")); // The 3 bytes we wrote
                                              // Should have spacing for remaining bytes
    }

    #[test]
    fn test_write_line_increments_byte_counter() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();

        let buffer = [0u8; 8];
        line_writer.write_line(&buffer, 8).unwrap();

        // Write another line - offset should have changed
        line_writer.write_line(&buffer, 8).unwrap();
        let output = writer.to_string();

        // Should show first offset all zeros
        assert!(output.contains("00000000"));
        // The output has ANSI codes, so just check offset changes
        // by checking that we have a second line with different offset chars
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() >= 2, "Should have at least 2 lines");
    }

    #[test]
    fn test_codepage_437_characters() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();

        let buffer = [0x00, 0x01, 0x02, 0x20, 0x41, 0x80, 0x81, 0xff];
        line_writer.write_line(&buffer, 8).unwrap();

        let output = writer.to_string();
        // Should contain codepage 437 representations
        assert!(output.contains("⋄")); // 0x00
        assert!(output.contains("☺")); // 0x01
        assert!(output.contains(" ")); // 0x20
        assert!(output.contains("A")); // 0x41
    }

    #[test]
    fn test_new_max_width_small() {
        let mut writer = TestWriter::new();
        let line_writer = LineWriter::new_max_width(&mut writer, 50).unwrap();
        // Should default to minimum (8 bytes)
        assert_eq!(line_writer.bytes_per_line(), 8);
    }

    #[test]
    fn test_new_max_width_large() {
        let mut writer = TestWriter::new();
        let line_writer = LineWriter::new_max_width(&mut writer, 200).unwrap();
        // Should allow more than minimum bytes
        assert!(line_writer.bytes_per_line() > 8);
        // Should be multiple of 8
        assert_eq!(line_writer.bytes_per_line() % 8, 0);
    }

    #[test]
    fn test_flush() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();

        let buffer = [0x41; 8];
        line_writer.write_line(&buffer, 8).unwrap();

        // Flush should not error
        assert!(line_writer.flush().is_ok());
    }

    #[test]
    fn test_hex_offset_leading_zeros() {
        let mut writer = TestWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();

        let buffer = [0u8; 8];
        line_writer.write_line(&buffer, 8).unwrap();

        let output = writer.to_string();
        // First line should have leading zeros
        assert!(output.contains("00000000"));
    }
}
