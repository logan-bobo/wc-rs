use core::str;
use std::fs;
use std::error::Error;

use clap::Parser;

const NEWLINE: u8 = 10;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'b', required = false)]
    pub count_bytes: bool,

    #[arg(short = 'l', required = false)]
    pub count_lines: bool,

    #[arg(short = 'w', required = false)]
    pub count_words: bool,

    #[arg(required = true)]
    pub file_name: String,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // What if the file is large?
    let bytes = fs::read(&args.file_name)?;

    if args.count_bytes {
        println!("\t{} {}", &args.file_name, bytes.len());
    }

    if args.count_lines {
        let lines = count_lines(&bytes);

        println!("\t{} {}", &args.file_name, lines);
    }

    if args.count_words  {
        let words = count_words(&bytes);

        println!("\t{} {}", &args.file_name, words);
    }

    Ok(())
}


fn count_lines(bytes: &Vec<u8>) -> u32 {
    let mut new_line_count: u32 = 0;

    for byte in bytes {
        if *byte == NEWLINE {
            new_line_count += 1;
        }
    }

    new_line_count
}

fn count_words(bytes: &Vec<u8>) -> usize {
    // we can handle this better
    let words_as_string: &str = str::from_utf8(bytes).expect("utf-8 error");

    words_as_string.split_ascii_whitespace().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_new_lines_in_file() {
        // UTF-8 newline as dec is 10
        let content = vec![10, 10, 10];
        let expected_lines = 3;

        assert_eq!(expected_lines, count_lines(&content));
    }

    #[test]
    fn count_words_in_file() {
        let content: Vec<u8> = "Hello\t World \n \n Today!   All!".as_bytes().to_vec();
        let expected_words: usize = 4;

        assert_eq!(expected_words, count_words(&content));
    }
}
