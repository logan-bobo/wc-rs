use core::str;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::str::Utf8Error;

use clap::Parser;

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

#[derive(Debug)]
struct FileMetaData {
    lines: usize,
    chars: usize,
    bytes: usize,
    words: usize,
}

impl FileMetaData {
    fn new() -> Self {
        FileMetaData {
            lines: 0,
            chars: 0,
            bytes: 0,
            words: 0,
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_name = &args.file_name;
    let mut metadata = FileMetaData::new();

    let mut reader: Box<dyn BufRead> = if file_name.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        let f = fs::File::open(&args.file_name)?;
        Box::new(BufReader::new(f))
    };

    let mut line_buffer = Vec::<u8>::new();
    loop {
        line_buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut line_buffer)?;
        if bytes_read == 0 {
            break;
        }

        metadata.lines += 1;
        metadata.chars += count_chars(&line_buffer)?;
        metadata.bytes += bytes_read;
        metadata.words += count_words(&line_buffer)?;
    }

    if args.count_bytes {
        println!("  {} {}", metadata.bytes, file_name);
    }

    if args.count_lines {
        println!("  {} {}", metadata.lines, file_name);
    }

    if args.count_words {
        println!("  {} {}", metadata.words, file_name);
    }

    if args.count_chars {
        println!("  {} {}", metadata.chars, file_name);
    }

    if is_default_option(&args) {
        println!(
            "  {} {} {} {}",
            metadata.lines, metadata.words, metadata.bytes, file_name
        )
    }

    Ok(())
}

fn count_words(bytes: &[u8]) -> Result<usize, Utf8Error> {
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
