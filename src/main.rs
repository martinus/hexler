use clap::Parser;
use std::{io::prelude::*, time::Instant};

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

// https://de.wikipedia.org/wiki/Codepage_437
const CODE_PAGE_437: [char; 256] = [
    '␀', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', // 00-0f
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', // 10-1f
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', // 20-2f
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', // 30-3f
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 40-4f
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 50-5f
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 60-6f
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', // 70-7f
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', // 80-8f
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', // 90-9f
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', // a0-af
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', // b0-bf
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', // c0-cf
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', // d0-df
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', // e0-ef
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ', // f0-ff
];

fn dump<R: std::io::Read>(mut reader: R) -> std::io::Result<()> {
    // first, create a hex lookup table


    let lookup_table: Vec<String> = CODE_PAGE_437.iter().map(|c| c.to_string()).collect();
    let mut buffer = [0u8; 4096];
    let mut writer = std::io::BufWriter::new(std::io::stdout());

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for byte in &buffer[..bytes_read] {
            // let str = CODE_PAGE_437[*byte as usize].to_string();
            // let bytes = str.as_bytes();
            let bytes = lookup_table[*byte as usize].as_bytes();
            writer.write_all(&bytes).unwrap();
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let stdin = std::io::stdin();
    let start = Instant::now();
    let _ = dump(stdin.lock());
    let end = Instant::now();
    eprintln!("\n{:?} sec", end.duration_since(start));
}
