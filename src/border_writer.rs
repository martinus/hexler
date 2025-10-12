/// Handles drawing borders for the hex output
pub struct BorderWriter;

impl BorderWriter {
    const HORIZONTAL: &'static str = "─";
    const CONNECTOR_TOP: &'static str = "┬";
    const CONNECTOR_BOTTOM: &'static str = "┴";

    /// Write a header border with a title
    pub fn write_header<W: std::io::Write>(
        writer: &mut W,
        title: &str,
        bytes_per_line: usize,
    ) -> std::io::Result<()> {
        Self::write_border(writer, title, bytes_per_line, Self::CONNECTOR_TOP, true)
    }

    /// Write a footer border with a title
    pub fn write_footer<W: std::io::Write>(
        writer: &mut W,
        title: &str,
        bytes_per_line: usize,
    ) -> std::io::Result<()> {
        Self::write_border(writer, title, bytes_per_line, Self::CONNECTOR_BOTTOM, false)
    }

    fn write_border<W: std::io::Write>(
        writer: &mut W,
        title: &str,
        bytes_per_line: usize,
        connector: &str,
        title_first: bool,
    ) -> std::io::Result<()> {
        let num_groups = bytes_per_line / 8;
        let num_bytes_per_group = 8 * 3 + 1; // 8 bytes * 3 chars each + 1 space

        if title_first && !title.is_empty() {
            writeln!(writer, "{}", title)?;
        }

        // Left section (offset column): 9 chars (8 hex + 1 space)
        for _ in 0..9 {
            writer.write_all(Self::HORIZONTAL.as_bytes())?;
        }
        writer.write_all(connector.as_bytes())?;

        // Middle section (hex bytes)
        for _ in 0..(num_groups * num_bytes_per_group) {
            writer.write_all(Self::HORIZONTAL.as_bytes())?;
        }
        writer.write_all(connector.as_bytes())?;

        // Right section (ASCII representation)
        for _ in 0..(bytes_per_line + 1) {
            writer.write_all(Self::HORIZONTAL.as_bytes())?;
        }

        if !title_first && !title.is_empty() {
            writeln!(writer, "{}", title)?;
        } else {
            writeln!(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_header() {
        let mut output = Vec::new();
        BorderWriter::write_header(&mut output, "Test Header", 8).unwrap();
        
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("Test Header"));
        assert!(result.contains("─"));
        assert!(result.contains("┬"));
    }

    #[test]
    fn test_write_footer() {
        let mut output = Vec::new();
        BorderWriter::write_footer(&mut output, "Test Footer", 8).unwrap();
        
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("Test Footer"));
        assert!(result.contains("─"));
        assert!(result.contains("┴"));
    }

    #[test]
    fn test_empty_title() {
        let mut output = Vec::new();
        BorderWriter::write_header(&mut output, "", 8).unwrap();
        
        let result = String::from_utf8_lossy(&output);
        assert!(result.contains("─"));
        assert!(result.contains("┬"));
    }

    #[test]
    fn test_different_widths() {
        let mut output = Vec::new();
        BorderWriter::write_header(&mut output, "Test", 16).unwrap();
        let len_16 = output.len();

        output.clear();
        BorderWriter::write_header(&mut output, "Test", 32).unwrap();
        let len_32 = output.len();

        assert!(len_32 > len_16, "32-byte width should produce longer border");
    }
}
