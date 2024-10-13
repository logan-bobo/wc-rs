use core::str;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::str::Utf8Error;

use clap::Parser;

const NEWLINE: u8 = 10;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        short = 'c',
        required = false,
        help = "Count the bytes from a file or stdin"
    )]
    pub count_bytes: bool,

    #[arg(
        short = 'l',
        required = false,
        help = "Count the lines from a file or stdin"
    )]
    pub count_lines: bool,

    #[arg(
        short = 'w',
        required = false,
        help = "Count the words from a file a stdin"
    )]
    pub count_words: bool,

    #[arg(
        short = 'm',
        required = false,
        help = "Count the characters from a file or stdin"
    )]
    pub count_chars: bool,

    #[arg(
        help = "The path of the file you wish to use, for example ~/file.txt",
        required = false,
        default_value = ""
    )]
    pub file_name: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_name = &args.file_name;

    let bytes: Vec<u8> = if file_name.is_empty() {
        let mut bytes_vect: Vec<u8> = vec![];
        let data = io::stdin().bytes();

        for byte in data {
            bytes_vect.push(byte?);
        }

        bytes_vect
    } else {
        fs::read(&args.file_name)?
    };

    if args.count_bytes {
        println!("  {} {}", bytes.len(), file_name);
    }

    if args.count_lines {
        println!("  {} {}", count_lines(&bytes), file_name);
    }

    if args.count_words {
        let words = count_words(&bytes)?;

        println!("  {} {}", words, file_name);
    }

    if args.count_chars {
        let chars = count_chars(&bytes)?;

        println!("  {} {}", chars, file_name);
    }

    if is_default_option(&args) {
        let words = count_words(&bytes)?;

        println!(
            "  {} {} {} {}",
            count_lines(&bytes),
            words,
            bytes.len(),
            file_name
        )
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

fn count_words(bytes: &[u8]) -> Result<usize, Utf8Error> {
    // we can handle this better
    let words_as_string: &str = str::from_utf8(bytes)?;

    Ok(words_as_string.split_ascii_whitespace().count())
}

fn count_chars(bytes: &[u8]) -> Result<usize, Utf8Error> {
    let words_as_string: &str = str::from_utf8(bytes)?;

    Ok(words_as_string.chars().count())
}

fn is_default_option(args: &Args) -> bool {
    if !args.count_bytes && !args.count_lines && !args.count_words && !args.count_chars {
        return true;
    }

    false
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

        assert_eq!(expected_words, count_words(&content).unwrap());
    }

    #[test]
    fn count_chars_in_file() {
        let content = "Hello World".as_bytes().to_vec();
        let expected_chars = 11;

        assert_eq!(expected_chars, count_chars(&content).unwrap())
    }
}
