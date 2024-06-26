pub mod byte_to_color;
pub mod line_writer;

use chrono::{DateTime, Local};
use pager::Pager;
use size::Size;
use std::fs;

use clap::Parser;
use line_writer::LineWriter;

#[derive(Parser, Debug)]
#[command(author, version, about="A colorful hex printer with opinionated defaults", long_about = None)]
struct Args {
    /// Number of bytes per line. Must be multiple of 8
    #[arg(short, long)]
    num_bytes_per_line: Option<usize>,

    /// Disables pager and write all output to stdout
    #[arg(short, long, default_value_t = false)]
    stdout: bool,

    /// Writes bytes 0 to 255, only for demonstration purposes
    #[arg(long, default_value_t = false)]
    demo: bool,

    /// The file to display. If none is provided, the standard input (stdin) will be used instead.
    #[arg()]
    file: Option<std::path::PathBuf>,
}

// default format: 32 bytes per line
// 001428d0: f30f 1efa 5548 89e5  4156 4154 5348 81ec  8800 0000 4c8b 364c  8b67 0864 488b 0425  ....UH..AVATSH......L.6L.g.dH..%
fn dump<R: std::io::Read, W: std::io::Write>(
    title: &str,
    mut reader: R,
    line_writer: &mut LineWriter<W>,
) -> std::io::Result<()> {
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
    line_writer.flush()
}

/**
 * This demo dumps the bytes 0 to 255 to stdout.
 */
#[allow(clippy::needless_range_loop)]
fn demo<W: std::io::Write>(line_writer: &mut LineWriter<W>) -> std::io::Result<()> {
    let mut arr = [0u8; 256];
    for i in 0..256 {
        arr[i] = i as u8;
    }

    // we need to use Cursor so we get an std::io::Reader
    let reader = std::io::Cursor::new(arr);
    dump("demo, 256 bytes, 0 to 255", reader, line_writer)
}

fn run() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let mut writer = std::io::BufWriter::new(std::io::stdout());

    // determine terminal size, and from that the number of bytes to print per line.
    let line_writer = match args.num_bytes_per_line {
        Some(num_bytes) => LineWriter::new_bytes(&mut writer, num_bytes),
        None => LineWriter::new_max_width(&mut writer, term_size::dimensions().unwrap().0),
    };

    let mut line_writer = line_writer.unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

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
                "[1m{}[0m   {}   {}",
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

fn main() {
    let result = run();
    if let Err(err) = result {
        if err.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("Error: {err:?}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    pub struct BufferWriter {
        data: Vec<u8>,
    }

    impl BufferWriter {
        pub fn new() -> Self {
            BufferWriter { data: vec![] }
        }

        pub fn to_utf8(&self) -> Result<&str, Utf8Error> {
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

    use std::str::Utf8Error;

    use super::*;

    #[test]
    fn test_dump_empty() {
        let mut reader = std::io::Cursor::new(b"x");

        let mut writer = BufferWriter::new();
        let mut line_writer = LineWriter::new_max_width(&mut writer, 8).unwrap();
        dump("Test", &mut reader, &mut line_writer).unwrap();
        println!("data={}", writer.to_utf8().unwrap());
    }
}
