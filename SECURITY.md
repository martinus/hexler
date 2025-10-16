# Security Policy

## Supported Versions

We support the latest release of hexler with security updates.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in hexler, please report it by:

1. **DO NOT** open a public issue
2. Email the maintainer directly at [your-email@example.com] with:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggested fixes

You should receive a response within 48 hours. If the vulnerability is confirmed, we will:

- Work on a fix as quickly as possible
- Release a security update
- Credit you in the release notes (unless you prefer to remain anonymous)

## Security Considerations

hexler is a command-line tool that:
- Reads files from the filesystem
- Processes binary data
- Outputs to terminal

### Best Practices

- Only run hexler on trusted input files
- Be cautious when viewing files from untrusted sources
- The tool does not execute any code from the files it reads
- Terminal escape sequences are not interpreted in the ASCII display

Thank you for helping keep hexler secure!
