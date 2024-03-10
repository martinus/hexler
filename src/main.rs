pub mod byte_to_color;
pub mod line_writer;

use clap::Parser;
use line_writer::LineWriter;
use std::{io, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about="A colorful hex printer", long_about = None)]
struct Args {
    /// Number of bytes per line. Must be multiple of 8
    #[arg(short, long, default_value_t = 16)]
    num_bytes: usize,

    /// Demonstrate output with each byte
    #[arg(long, default_value_t = false)]
    demo: bool,
}

// default format: 32 bytes per line
// 001428d0: f30f 1efa 5548 89e5  4156 4154 5348 81ec  8800 0000 4c8b 364c  8b67 0864 488b 0425  ....UH..AVATSH......L.6L.g.dH..%
fn dump<R: std::io::Read>(mut reader: R, num_bytes_per_line: usize) -> std::io::Result<()> {
    // first, create a hex lookup table
    let mut buffer = [0u8; 4096];

    let mut line_writer = LineWriter::new(num_bytes_per_line);

    let mut num_bytes_in_line = 0;
    let mut line_buffer = [0u8; 128];
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

fn demo() -> std::io::Result<()> {
    let mut arr = [0u8; 256];
    for i in 0..256 {
        arr[i] = i as u8;
    }
    let reader = io::Cursor::new(arr);
    dump(reader, 16)
}

fn run() -> std::io::Result<()> {
    // TODO read from file if specified in args, use maximum width
    let args: Args = Args::parse();

    if args.demo {
        return demo();
    }

    if args.num_bytes % 8 != 0 || args.num_bytes < 8 {
        eprintln!(
            "Error: num-bytes must be multiple of 8 and a minimum of 8, but it's {}",
            args.num_bytes
        );
        std::process::exit(1);
    }

    let stdin = std::io::stdin();
    dump(stdin.lock(), args.num_bytes)
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
