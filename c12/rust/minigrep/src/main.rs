use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args() is an iterator. This can be collect()'d into a vector.
    // args[0] is the executable path like a C program (even with cargo run).
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else() runs on Result, and either assigns the value or runs the block with the error type.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Regular println! but for writing to stderr.
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Since there's no OK value that we need to use, we can detect the error the old-school way.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}