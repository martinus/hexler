use clap::Parser;
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn create_binary_file() -> std::io::Result<()> {
    let mut file = File::create("bytes.bin")?;

    // Create a byte array with values from 0 to 255
    let bytes: Vec<u8> = (0..=255).collect();

    // Write the byte array to the file
    file.write_all(&bytes)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    match create_binary_file() {
        Ok(()) => println!("Binary file created successfully!"),
        Err(err) => println!("Error creating file: {}", err),
    }
    println!("Hello, world!");
}
