use std::io::{self, Write};

fn main() {
    let affirmation = {
        let args: Vec<_> = std::env::args().skip(1).collect();

        // Test for the "--help" || "-h" flags and print usage and exit if either are present
        if args.iter().any(|a| a == "--help" || a == "-h") {
            eprintln!("Usage: ryes [affirmation]");
            eprintln!("Outputs \"y\" (by default) or [affirmation], forever.");
            eprintln!("Inspired by Unix yes");
            std::process::exit(0)
        }

        // Collect [affirmation] from the arguments or use "y" as the default
        let afm = if args.is_empty() {
            "y".into()
        } else {
            args.join(" ")
        };

        // Convert to bytes and append a newline
        let mut bytes = afm.into_bytes();
        bytes.push(b'\n');
        bytes
    };

    // Create a buffered writer with a 64KB buffer (like GNU yes)
    let mut out = io::BufWriter::with_capacity(64 * 1024, io::stdout().lock());

    loop {
        if let Err(e) = out.write_all(&affirmation) {
            // Catch broken pipe errors and gracefully exit
            if e.kind() == io::ErrorKind::BrokenPipe {
                break;
            } else {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}
