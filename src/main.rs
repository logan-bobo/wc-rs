use std::process;

use clap::Parser; // can we move this to the lib so the main only depends on the lib

use wc_rust::Args;


fn main() {
    let args = Args::parse();

    println!("file - {}", args.count_bytes);

    if let Err(e) = wc_rust::run(args) {
        eprint!("Application Error: {e}");
        process::exit(1);
    }
}
