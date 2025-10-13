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
/// Uses multi-threading with double buffering to overlap I/O operations: while one
/// buffer is being written to output in a separate thread, the main thread reads and
/// processes the next chunk of data into the other buffer. Buffers are recycled
/// between threads to avoid allocations.
///
/// # Arguments
/// * `title` - Header text to display (filename, "stdin", etc.)
/// * `reader` - Data source to read from
/// * `line_writer` - Configured line writer for output formatting
/// * `writer` - Output writer to write the formatted data to
pub fn dump<R: std::io::Read, W: std::io::Write + Send + 'static>(
    title: &str,
    mut reader: R,
    line_writer: &mut LineWriter,
    writer: W,
) -> Result<()> {
    use std::sync::mpsc;
    use std::thread;

    const MAX_READ_BUFFER_SIZE: usize = 64 * 1024; // 64KB chunks for better I/O performance

    let bytes_per_line = line_writer.bytes_per_line();

    // Make sure the buffer size is a multiple of bytes_per_line, otherwise we would print partial lines.
    // When reading data we make sure to always fill the buffer completely except for the last read.
    let mut buffer = vec![0u8; (MAX_READ_BUFFER_SIZE / bytes_per_line) * bytes_per_line];

    // Double buffering: two output buffers that we recycle between threads
    let mut output_buffer_a = Vec::with_capacity(bytes_per_line * 10);
    let output_buffer_b = Vec::with_capacity(bytes_per_line * 10);

    // Bidirectional channels for buffer exchange
    let (write_tx, write_rx) = mpsc::sync_channel::<Vec<u8>>(0); // Send buffers to writer
    let (return_tx, return_rx) = mpsc::sync_channel::<Vec<u8>>(1); // Get buffers back

    // Spawn writer thread - runs in parallel with reading/processing
    let writer_handle = thread::spawn(move || -> std::io::Result<()> {
        let mut writer = writer;
        for mut buf in write_rx {
            writer.write_all(&buf)?;
            // Return the buffer for reuse (clear it first)
            buf.clear();
            if return_tx.send(buf).is_err() {
                break; // Main thread dropped the receiver
            }
        }
        writer.flush()?;
        Ok(())
    });

    // Use buffer A for the header
    line_writer.write_border(&mut output_buffer_a, line_writer::Border::Header, title)?;

    // Send first buffer, get started
    if write_tx.send(output_buffer_a).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "Writer thread disconnected",
        )
        .into());
    }

    // Current working buffer
    let mut current_buffer = output_buffer_b;

    loop {
        // Read until buffer is full or EOF - this ensures we only get partial lines at the very end
        let mut total_read = 0;
        while total_read < buffer.len() {
            let bytes_read = reader.read(&mut buffer[total_read..])?;
            if bytes_read == 0 {
                break; // EOF reached
            }
            total_read += bytes_read;
        }

        if total_read == 0 {
            break; // Nothing more to read
        }

        // Process bytes in chunks aligned to line boundaries
        let data = &buffer[..total_read];
        let mut offset = 0;

        while offset < total_read {
            let available = total_read - offset;
            let to_write = available.min(bytes_per_line);

            // Write directly from the buffer slice, no copying needed
            line_writer.write_line(
                &mut current_buffer,
                &data[offset..offset + to_write],
                to_write,
            );
            offset += to_write;
        }

        // Send current buffer to writer thread
        if write_tx.send(current_buffer).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Writer thread disconnected",
            )
            .into());
        }

        // Get a recycled buffer back (blocks until writer is done with previous buffer)
        current_buffer = return_rx.recv().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Writer thread disconnected")
        })?;
    }

    // Add footer to current buffer
    line_writer.write_border(&mut current_buffer, line_writer::Border::Footer, "")?;

    // Send final buffer
    if write_tx.send(current_buffer).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "Writer thread disconnected",
        )
        .into());
    }

    // Close write channel and wait for writer thread to finish all writes
    drop(write_tx);
    drop(return_rx); // Close return channel too
    writer_handle.join().unwrap()?;

    Ok(())
}
/// Demo mode: outputs bytes 0-255 to demonstrate all possible byte values and their colors.
#[allow(clippy::needless_range_loop)]
pub fn demo<W: std::io::Write + Send + 'static>(
    line_writer: &mut LineWriter,
    writer: W,
) -> Result<()> {
    let mut arr = [0u8; 256];
    for i in 0..arr.len() {
        arr[i] = i as u8;
    }

    // we need to use Cursor so we get an std::io::Reader
    let reader = std::io::Cursor::new(arr);
    dump("demo, 256 bytes, 0 to 255", reader, line_writer, writer)
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

    let writer = std::io::stdout();

    // determine terminal size, and from that the number of bytes to print per line.
    let line_writer = match args.num_bytes_per_line {
        Some(num_bytes) => LineWriter::new_bytes(num_bytes),
        None => {
            let term_width = term_size::dimensions()
                .ok_or(HexlerError::TerminalSizeError)?
                .0;
            LineWriter::new_max_width(term_width)
        }
    };

    let mut line_writer = line_writer?;

    // use less as the pager, much like git
    if !args.stdout {
        Pager::with_pager("less --raw-control-chars --quit-if-one-screen").setup();
    }

    if args.demo {
        return demo(&mut line_writer, writer);
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
            dump(title.as_str(), f?, &mut line_writer, writer)
        }
        None => dump("stdin", std::io::stdin().lock(), &mut line_writer, writer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Test helper: A thread-safe writer that stores output in a Vec<u8> for verification.
    #[derive(Clone)]
    pub struct BufferWriter {
        data: Arc<Mutex<Vec<u8>>>,
    }

    impl BufferWriter {
        pub fn new() -> Self {
            BufferWriter {
                data: Arc::new(Mutex::new(vec![])),
            }
        }

        pub fn get_output(&self) -> Vec<u8> {
            self.data.lock().unwrap().clone()
        }

        pub fn get_output_as_string(&self) -> String {
            String::from_utf8_lossy(&self.get_output()).to_string()
        }
    }

    impl std::io::Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_dump_empty() {
        let test_data = b"";
        let mut reader = std::io::Cursor::new(test_data);

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let result = dump("Empty", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Should have header and footer borders
        assert!(output.contains("Empty"));
        assert!(output.contains("─")); // Border character
    }

    #[test]
    fn test_dump_single_byte() {
        let test_data = b"x";
        let mut reader = std::io::Cursor::new(test_data);

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let result = dump("Test", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Should contain the hex value of 'x' (0x78)
        assert!(output.contains("78"));
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_dump_multiple_lines() {
        // Test data that spans multiple lines
        let test_data: Vec<u8> = (0..=255).collect();
        let mut reader = std::io::Cursor::new(&test_data);

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(16).unwrap();

        let result = dump("Multi-line test", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Should have title and multiple lines of hex output
        assert!(output.contains("Multi-line test"));
        // Should contain hex for first byte (00) and last byte (ff)
        assert!(output.contains("00"));
        assert!(output.contains("ff"));
        // Count lines - should have header, footer, and data lines
        let line_count = output.lines().count();
        assert!(line_count >= 18); // 256 bytes / 16 per line = 16 lines + header + footer
    }

    #[test]
    fn test_dump_partial_line() {
        // Test data that doesn't fill a complete line
        let test_data = b"Hello";
        let mut reader = std::io::Cursor::new(test_data);

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(16).unwrap();

        let result = dump("Partial", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Should contain hex values for "Hello"
        assert!(output.contains("48")); // 'H'
        assert!(output.contains("65")); // 'e'
        assert!(output.contains("6c")); // 'l'
        assert!(output.contains("6f")); // 'o'
    }

    #[test]
    fn test_line_writer_invalid_bytes_per_line() {
        // Test less than minimum
        let result = LineWriter::new_bytes(4);
        assert!(result.is_err());

        // Test not multiple of 8
        let result = LineWriter::new_bytes(12);
        assert!(result.is_err());

        // Test valid values
        assert!(LineWriter::new_bytes(8).is_ok());
        assert!(LineWriter::new_bytes(16).is_ok());
        assert!(LineWriter::new_bytes(24).is_ok());
    }

    #[test]
    fn test_line_writer_max_width_calculation() {
        // Small width should give minimum bytes
        let line_writer = LineWriter::new_max_width(50).unwrap();
        assert_eq!(line_writer.bytes_per_line(), 8);

        // Larger width should give more bytes
        let line_writer = LineWriter::new_max_width(150).unwrap();
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

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let result = dump("Various bytes", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Verify all hex values are present
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

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(8).unwrap();

        let result = dump("Border Test", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output_as_string();
        // Should have border characters and title
        assert!(output.contains("Border Test"));
        assert!(output.contains("─")); // Unicode box drawing character
        let line_count = output.lines().count();
        assert!(line_count >= 3); // At least header, data, and footer
    }

    #[test]
    fn test_exact_buffer_boundary() {
        // Test data that exactly fills the read buffer (64KB)
        const READ_SIZE: usize = 64 * 1024;
        let test_data: Vec<u8> = (0..READ_SIZE).map(|i| (i % 256) as u8).collect();
        let mut reader = std::io::Cursor::new(&test_data);

        let writer = BufferWriter::new();
        let writer_clone = writer.clone();
        let mut line_writer = LineWriter::new_bytes(16).unwrap();

        let result = dump("Buffer boundary", &mut reader, &mut line_writer, writer);
        assert!(result.is_ok());

        let output = writer_clone.get_output();
        // Should have produced output
        assert!(!output.is_empty());
        // Should have multiple lines (64KB / 16 bytes per line = 4096 lines + borders)
        let output_str = String::from_utf8_lossy(&output);
        let line_count = output_str.lines().count();
        assert!(line_count >= 4098); // 4096 data lines + header + footer
    }
}
