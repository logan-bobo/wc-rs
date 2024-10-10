use core::str;
use std::{fs, error::Error};

use clap::Parser;

const NEWLINE: u8 = 10;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'c', required = false)]
    pub count_bytes: bool,

    #[arg(short = 'l', required = false)]
    pub count_lines: bool,

    #[arg(short = 'w', required = false)]
    pub count_words: bool,

    #[arg(short = 'm', required = false)]
    pub count_chars: bool,

    #[arg(required = true)]
    pub file_name: String,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // What if the file is large? (oom)
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

    if args.count_chars {
        let chars = count_chars(&bytes);
        println!("\t{} {}", &args.file_name, chars)
    }

    Ok(())
}

fn count_lines(bytes: &Vec<u8>) -> usize {
    let mut new_line_count = 0;

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

fn count_chars(bytes: &Vec<u8>) -> usize {
    let words_as_string: &str = str::from_utf8(&bytes).expect("utf-8 error");

    words_as_string.chars().count()
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
        let expected_words = 4;

        assert_eq!(expected_words, count_words(&content));
    }

    #[test]
    fn count_chars_in_file() {
        let content = "Hello World".as_bytes().to_vec();
        let expected_chars = 11;

        assert_eq!(expected_chars, count_chars(&content))
    }
}
