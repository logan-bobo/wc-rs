use std::fs;
use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub count_bytes: String,
}


pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // We dont need a test for this becasue it is stdlib functionality
    if args.count_bytes != ""{
        let bytes = fs::read(args.count_bytes)?;

        println!("{}", bytes.len());
    }

    Ok(())
}

