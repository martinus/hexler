pub mod line_writer;
pub mod byte_to_color;

use clap::Parser;
use line_writer::LineWriter;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Demonstrate output with each byte
    #[arg(long, default_value_t = false)]
    example: bool,
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

    Ok(())
}

/**
 *
 */
fn main() {
    // TODO read from file if specified in args, use maximum width
    let args: Args = Args::parse();

    let stdin: std::io::Stdin = std::io::stdin();
    let start: Instant = Instant::now();
    let _ = dump(stdin.lock(), 16);
    let end: Instant = Instant::now();
    eprintln!("\n{:?} sec", end.duration_since(start));
}
