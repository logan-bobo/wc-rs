use std::fs;
use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, default_value_t = String::new())]
    pub count: String,

    #[arg(short, default_value_t = String::new())]
    pub lines: String,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // We dont need a test for this becasue we are just using the stdlib to count bytes
    if args.count != ""{
        let bytes = fs::read(&args.count)?;

        println!("\t{} {}", args.count, bytes.len());
    }

    if args.lines != "" {
        let bytes = fs::read(&args.lines)?;

        let lines = count_lines(&bytes);

        println!("\t{} {}", args.lines, lines);
    }

    Ok(())
}


fn count_lines(bytes: &Vec<u8>) -> u32 {
    let mut new_line_count: u32 = 0;

    for byte in bytes {
        if *byte == 10 {
            new_line_count += 1;
        }
    }

    new_line_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_new_lines() {
        // UTF-8 newline as dec is 10
        let content = vec![10, 10, 10];
        let expected_lines = 3;

        assert_eq!(expected_lines, count_lines(&content));
    }
}
