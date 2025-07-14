use std::io::{self, Write};

fn main() {
    // Assemble the line
    let line = {
        // Retrieve [STRINGS] from command line arguments or use 'y' as default
        let args: Vec<_> = std::env::args().skip(1).collect();

        // Test for the `--help` flag and print usage and die if present 
        if args.contains(&"--help".to_string()) {
            eprintln!("Usage: ryes [STRINGS]");
            eprintln!("{}Prints the provided strings repeatedly, or 'y' if no strings are given.", " ".repeat(7));
            eprintln!("\nExample: ryes hello world");
            std::process::exit(1);
        }

        let s = if args.is_empty() {
            "y".into()
        } else {
            args.join(" ")
        };

        // Convert to bytes and append a newline
        let mut bytes = s.into_bytes();
        bytes.push(b'\n');
        bytes
    };

    // Create a buffered writer with a 64KB buffer (like GNU yes)
    let mut out = io::BufWriter::with_capacity(64 * 1024, io::stdout().lock());

    loop {
        out.write_all(&line).unwrap();
    }
}
