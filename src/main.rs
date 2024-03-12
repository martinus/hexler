pub mod byte_to_color;
pub mod line_writer;

use size::Size;
use std::{fs, io::Read, os::{linux::fs::MetadataExt, unix::fs::FileTypeExt}};

use clap::Parser;
use line_writer::LineWriter;

#[derive(Parser, Debug)]
#[command(author, version, about="A colorful hex printer", long_about = None)]
struct Args {
    /// Number of bytes per line. Must be multiple of 8
    #[arg(short, long)]
    num_bytes: Option<usize>,

    /// Demonstrate output with each byte
    #[arg(long, default_value_t = false)]
    demo: bool,

    /// The file to display. If none is provided, the standard input (stdin) will be used instead.
    #[arg()]
    file: Option<std::path::PathBuf>,
}

// default format: 32 bytes per line
// 001428d0: f30f 1efa 5548 89e5  4156 4154 5348 81ec  8800 0000 4c8b 364c  8b67 0864 488b 0425  ....UH..AVATSH......L.6L.g.dH..%
fn dump<R: std::io::Read>(
    title: &str,
    mut reader: R,
    num_bytes_per_line: usize,
) -> std::io::Result<()> {
    // first, create a hex lookup table
    let mut buffer = [0u8; 4096];

    let mut line_writer = LineWriter::new(num_bytes_per_line);

    let mut num_bytes_in_line = 0;
    let mut line_buffer = [0u8; 1024];

    line_writer.write_header(title)?;

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
            if num_bytes_in_line == num_bytes_per_line {
                line_writer.write_line(&line_buffer, num_bytes_in_line)?;
                num_bytes_in_line = 0
            }
        }
    }
    line_writer.write_line(&line_buffer, num_bytes_in_line)?;
    line_writer.flush()
}

fn demo(num_bytes_per_line: usize) -> std::io::Result<()> {
    let mut arr = [0u8; 256];
    for i in 0..256 {
        arr[i] = i as u8;
    }
    let reader = std::io::Cursor::new(arr);
    dump("demo, 256 bytes, 0 to 255", reader, num_bytes_per_line)
}

// Given maximum terminal width, calculate the number of bytes to print per line that just fits.

fn calc_num_bytes(max_width: usize) -> usize {
    let mut num_groups_of_8: usize = 1;
    while 13 + (num_groups_of_8 + 1) * 33 <= max_width {
        num_groups_of_8 += 1;
    }
    return num_groups_of_8 * 8;
}

fn run() -> std::io::Result<()> {
    let args: Args = Args::parse();

    // determine number of bytes per line
    let num_bytes = args
        .num_bytes
        .unwrap_or_else(|| calc_num_bytes(term_size::dimensions().unwrap().0));
    if num_bytes % 8 != 0 || num_bytes < 8 {
        eprintln!(
            "Error: num-bytes must be multiple of 8 and a minimum of 8, but it's {}",
            num_bytes
        );
        std::process::exit(1);
    }

    if args.demo {
        return demo(num_bytes);
    }

    match args.file {
        Some(file) => {
            let md = fs::metadata(&file)?;
            let size = Size::from_bytes(md.len());
            let t: time::OffsetDateTime = md.modified().unwrap().into();
            let title = format!(
                "[1m{}[0m  {}, {} {} {} {:0>2}:{:0>2}:{:0>2}",
                &file.display(),
                size,
                t.day(),
                t.month(),
                t.year(),
                t.hour(),
                t.minute(),
                t.second()
            );

            let f = std::fs::File::open(&file);
            dump(title.as_str(), f?, num_bytes)
        }
        None => dump("stdin", std::io::stdin().lock(), num_bytes),
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
