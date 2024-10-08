/**
 * 
 * 
Usage: grep [OPTIONS] <pattern> <files...>
Options:
-i                Case-insensitive search
-n                Print line numbers
-v                Invert match (exclude lines that match the pattern)
-r                Recursive directory search
-f                Print filenames
-c                Enable colored output
-h, --help        Show help information
 * 
 * 
*/
use std::env;
use std::process;
use grep::Config;  // Import the Config struct from your grep module

fn main() {
    // Parse command-line arguments and build the Config struct
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // If the run function returns an error, handle it
    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


