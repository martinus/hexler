use hexler::error::HexlerError;

fn main() {
    let result = hexler::run();
    if let Err(err) = result {
        // Ignore broken pipe errors (e.g., when piping to head)
        if let HexlerError::Io(io_err) = &err {
            if io_err.kind() == std::io::ErrorKind::BrokenPipe {
                return;
            }
        }
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
