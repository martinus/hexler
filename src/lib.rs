pub mod ascii_renderer;
pub mod border_writer;
pub mod byte_to_color;
pub mod error;
pub mod hex_formatter;
pub mod line_writer;

use chrono::{DateTime, Local};
use pager::Pager;
use size::Size;
use std::fs;

use clap::Parser;
use error::{HexlerError, Result};
use line_writer::LineWriter;

/// Command-line arguments for hexler.
#[derive(Parser, Debug)]
#[command(author, version, about="A colorful hex printer with opinionated defaults", long_about = None)]
pub struct Args {
    /// Number of bytes per line. Must be multiple of 8
    #[arg(short, long)]
    pub num_bytes_per_line: Option<usize>,

    /// Disables pager and write all output to stdout
    #[arg(short, long, default_value_t = false)]
    pub stdout: bool,

    /// Writes bytes 0 to 255, only for demonstration purposes
    #[arg(long, default_value_t = false)]
    pub demo: bool,

    /// The file to display. If none is provided, the standard input (stdin) will be used instead.
    #[arg()]
    pub file: Option<std::path::PathBuf>,
}

/// Reads data from a reader and outputs a colored hex dump.
///
/// The data is read in 4KB chunks for efficiency, then processed byte-by-byte
/// to build complete lines matching the configured bytes_per_line.
///
/// # Arguments
/// * `title` - Header text to display (filename, "stdin", etc.)
/// * `reader` - Data source to read from
/// * `line_writer` - Configured line writer for output formatting
pub fn dump<R: std::io::Read, W: std::io::Write>(
    title: &str,
    mut reader: R,
    line_writer: &mut LineWriter<W>,
) -> Result<()> {
    // first, create a hex lookup table
    let mut buffer = [0u8; 4096];

    let mut num_bytes_in_line = 0;
    let mut line_buffer = [0u8; 1024];

    line_writer.write_border(line_writer::Border::Header, title)?;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        // This loop is the most expensive part of this program.
        // It writes the hex representation of the bytes
        for byte in &buffer[..bytes_read] {
            line_buffer[num_bytes_in_line] = *byte;
            num_bytes_in_line += 1;
            if num_bytes_in_line == line_writer.bytes_per_line() {
                line_writer.write_line(&line_buffer, num_bytes_in_line)?;
                num_bytes_in_line = 0
            }
        }
    }
    line_writer.write_line(&line_buffer, num_bytes_in_line)?;
    line_writer.write_border(line_writer::Border::Footer, "")?;

    // make sure that the line writer is flushed to stdout before returning.
    line_writer.flush()?;
    Ok(())
}

/// Demo mode: outputs bytes 0-255 to demonstrate all possible byte values and their colors.
#[allow(clippy::needless_range_loop)]
pub fn demo<W: std::io::Write>(line_writer: &mut LineWriter<W>) -> Result<()> {
    let mut arr = [0u8; 256];
    for i in 0..256 {
        arr[i] = i as u8;
    }

    // we need to use Cursor so we get an std::io::Reader
    let reader = std::io::Cursor::new(arr);
    dump("demo, 256 bytes, 0 to 255", reader, line_writer)
}

/// Main application entry point - parses arguments and coordinates the hex dump output.
///
/// This function:
/// 1. Parses command-line arguments
/// 2. Determines terminal width and calculates optimal bytes_per_line (unless overridden)
/// 3. Sets up a pager (less) for interactive viewing (unless --stdout is used)
/// 4. Reads from a file or stdin and produces the hex dump
pub fn run() -> Result<()> {
    let args: Args = Args::parse();

    let mut writer = std::io::BufWriter::new(std::io::stdout());

    // determine terminal size, and from that the number of bytes to print per line.
    let line_writer = match args.num_bytes_per_line {
        Some(num_bytes) => LineWriter::new_bytes(&mut writer, num_bytes),
        None => {
            let term_width = term_size::dimensions()
                .ok_or(HexlerError::TerminalSizeError)?
                .0;
            LineWriter::new_max_width(&mut writer, term_width)
        }
    };

    let mut line_writer = line_writer?;

    // use less as the pager, much like git
    if !args.stdout {
        Pager::with_pager("less --raw-control-chars --quit-if-one-screen").setup();
    }

    if args.demo {
        return demo(&mut line_writer);
    }

    match args.file {
        // Reading from a known file, print its filename and it's last modified date
        Some(file) => {
            let md = fs::metadata(&file)?;
            let size = Size::from_bytes(md.len());
            let modified_time: DateTime<Local> = md.modified().unwrap().into();

            let mut file_name_str = format!("{}", file.display());
            if file_name_str.contains(' ') {
                file_name_str = format!("'{}'", file_name_str);
            }

            let title = format!(
                "\x1b[1m{}\x1b[0m   {}   {}",
                file_name_str,
                size,
                modified_time.format("%-d %b %Y %H:%M:%S")
            );

            let f = std::fs::File::open(&file);
            dump(title.as_str(), f?, &mut line_writer)
        }
        None => dump("stdin", std::io::stdin().lock(), &mut line_writer),
    }
}

#[cfg(test)]
mod tests {
    /// Test helper: A writer that stores output in a Vec<u8> for verification.
    pub struct BufferWriter {
        data: Vec<u8>,
    }

    impl BufferWriter {
        pub fn new() -> Self {
            BufferWriter { data: vec![] }
        }

        pub fn to_utf8(&self) -> std::result::Result<&str, std::str::Utf8Error> {
            std::str::from_utf8(&self.data)
        }
    }

    impl std::io::Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    use super::*;

    #[test]
    fn test_dump_empty() {
        let mut reader = std::io::Cursor::new(b"x");

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_max_width(&mut writer, 8).unwrap();
        dump("Test", &mut reader, &mut line_writer).unwrap();
        println!("data={}", writer.to_utf8().unwrap());
    }

    #[test]
    fn test_dump_multiple_lines() {
        // Test data that spans multiple lines
        let test_data: Vec<u8> = (0..=255).collect();
        let mut reader = std::io::Cursor::new(&test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 16).unwrap();
        dump("Multi-line test", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        assert!(output.contains("Multi-line test"));
        assert!(output.contains("00000000"));
        assert!(output.contains("f0")); // Last line contains f0
    }

    #[test]
    fn test_dump_partial_line() {
        // Test data that doesn't fill a complete line
        let test_data = b"Hello";
        let mut reader = std::io::Cursor::new(test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 16).unwrap();
        dump("Partial", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        assert!(output.contains("Partial"));
        assert!(output.contains("48 65 6c 6c 6f")); // "Hello" in hex
    }

    #[test]
    fn test_line_writer_invalid_bytes_per_line() {
        let mut writer = BufferWriter::new();
        
        // Test less than minimum
        let result = LineWriter::new_bytes(&mut writer, 4);
        assert!(result.is_err());
        
        // Test not multiple of 8
        let result = LineWriter::new_bytes(&mut writer, 12);
        assert!(result.is_err());
        
        // Test valid values
        assert!(LineWriter::new_bytes(&mut writer, 8).is_ok());
        assert!(LineWriter::new_bytes(&mut writer, 16).is_ok());
        assert!(LineWriter::new_bytes(&mut writer, 24).is_ok());
    }

    #[test]
    fn test_line_writer_max_width_calculation() {
        let mut writer = BufferWriter::new();
        
        // Small width should give minimum bytes
        let line_writer = LineWriter::new_max_width(&mut writer, 50).unwrap();
        assert_eq!(line_writer.bytes_per_line(), 8);
        
        // Larger width should give more bytes
        let line_writer = LineWriter::new_max_width(&mut writer, 150).unwrap();
        assert!(line_writer.bytes_per_line() >= 16);
    }

    #[test]
    fn test_dump_with_various_byte_values() {
        // Test with control characters, printable, and extended ASCII
        let test_data = vec![
            0x00, // NUL
            0x0a, // LF
            0x20, // Space
            0x41, // 'A'
            0x7f, // DEL
            0xff, // Extended
        ];
        let mut reader = std::io::Cursor::new(&test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();
        dump("Various bytes", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        assert!(output.contains("Various bytes"));
        // Check for individual hex values (with potential ANSI codes between them)
        assert!(output.contains("00"));
        assert!(output.contains("0a"));
        assert!(output.contains("20"));
        assert!(output.contains("41"));
        assert!(output.contains("7f"));
        assert!(output.contains("ff"));
    }

    #[test]
    fn test_border_output() {
        let test_data = b"test";
        let mut reader = std::io::Cursor::new(test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();
        dump("Border Test", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        // Check for border characters
        assert!(output.contains("─")); // horizontal line
        assert!(output.contains("┬")); // top connector
        assert!(output.contains("┴")); // bottom connector
        assert!(output.contains("│")); // vertical separator
    }

    #[test]
    fn test_empty_input() {
        let test_data = b"";
        let mut reader = std::io::Cursor::new(test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 8).unwrap();
        dump("Empty", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        assert!(output.contains("Empty"));
        // Should still have borders even with no data
        assert!(output.contains("─"));
    }

    #[test]
    fn test_exact_buffer_boundary() {
        // Test data that exactly fills the read buffer (4096 bytes)
        const READ_SIZE: usize = 4096;
        let test_data: Vec<u8> = (0..READ_SIZE).map(|i| (i % 256) as u8).collect();
        let mut reader = std::io::Cursor::new(&test_data);

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_bytes(&mut writer, 16).unwrap();
        dump("Buffer boundary", &mut reader, &mut line_writer).unwrap();
        
        let output = writer.to_utf8().unwrap();
        assert!(output.contains("Buffer boundary"));
    }
}
