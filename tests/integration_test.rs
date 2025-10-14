use std::io::Write;
use std::process::Command;

/// Integration tests for the hexler binary
#[test]
fn test_binary_exists() {
    let output = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--version")
        .output()
        .expect("Failed to execute hexler");

    assert!(output.status.success());
}

#[test]
fn test_help_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--help")
        .output()
        .expect("Failed to execute hexler");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hexler"));
    assert!(stdout.contains("--stdout"));
}

#[test]
fn test_basic_hex_output() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--stdout")
        .arg("--num-bytes-per-line")
        .arg("16")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn hexler");

    // Write test data
    let test_data = b"Hello, World!";
    cmd.stdin
        .as_mut()
        .unwrap()
        .write_all(test_data)
        .expect("Failed to write to stdin");

    let output = cmd.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that hex output contains expected hex values for "Hello"
    assert!(stdout.contains("48")); // 'H'
    assert!(stdout.contains("65")); // 'e'
    assert!(stdout.contains("6c")); // 'l'
    assert!(stdout.contains("6f")); // 'o'
}

#[test]
fn test_file_input() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Create a temporary file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_data = b"Test data for hexler\x00\x01\x02\x03";
    temp_file
        .write_all(test_data)
        .expect("Failed to write test data");
    temp_file.flush().expect("Failed to flush");

    let output = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--stdout")
        .arg("--num-bytes-per-line")
        .arg("16")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute hexler");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that output contains hex representation of the data
    assert!(stdout.contains("54")); // 'T'
    assert!(stdout.contains("65")); // 'e'
    assert!(stdout.contains("00")); // null byte
    assert!(stdout.contains("01")); // 0x01
    assert!(stdout.contains("02")); // 0x02
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--stdout")
        .arg("--num-bytes-per-line")
        .arg("16")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn hexler");

    // Close stdin immediately (empty input)
    drop(cmd.stdin.take());

    let output = cmd.wait_with_output().expect("Failed to read output");

    // Empty input should succeed (even if it produces no output)
    assert!(output.status.success());
}

#[test]
fn test_binary_data() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_hexler"))
        .arg("--stdout")
        .arg("--num-bytes-per-line")
        .arg("16")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn hexler");

    // Write binary data including all byte values
    let test_data: Vec<u8> = (0..=255).collect();
    cmd.stdin
        .as_mut()
        .unwrap()
        .write_all(&test_data)
        .expect("Failed to write to stdin");

    let output = cmd.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify some key byte values are present
    assert!(stdout.contains("00")); // 0x00
    assert!(stdout.contains("ff")); // 0xff
    assert!(stdout.contains("7f")); // 0x7f
}
