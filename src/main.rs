use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "ccwc")]
#[command(author = "Manmohan")]
#[command(version = "1.0")]
#[command(about = "A fast, memory-efficient clone of the Unix wc tool", long_about = None)]
struct Cli {
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    #[arg(short = 'l', long = "lines")]
    lines: bool,

    #[arg(short = 'w', long = "words")]
    words: bool,

    #[arg(short = 'm', long = "chars")]
    chars: bool,

    filename: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let filename_str = cli.filename.as_deref().unwrap_or("");

    let no_flags = !cli.bytes && !cli.lines && !cli.words && !cli.chars;

    if no_flags {
        let (lines, words, bytes) = if filename_str.is_empty() {
            count_all(io::stdin().lock())
        } else {
            count_all(BufReader::new(File::open(filename_str).unwrap()))
        };
        if filename_str.is_empty() {
            println!("{} {} {}", lines, words, bytes);
        } else {
            println!("{} {} {} {}", lines, words, bytes, filename_str);
        }
    } else if cli.bytes {
        let count = if filename_str.is_empty() {
            count_bytes(io::stdin().lock())
        } else {
            count_bytes(BufReader::new(File::open(filename_str).unwrap()))
        };
        print_result(count, filename_str);
    } else if cli.lines {
        let count = if filename_str.is_empty() {
            count_lines(io::stdin().lock())
        } else {
            count_lines(BufReader::new(File::open(filename_str).unwrap()))
        };
        print_result(count, filename_str);
    } else if cli.words {
        let count = if filename_str.is_empty() {
            count_words(io::stdin().lock())
        } else {
            count_words(BufReader::new(File::open(filename_str).unwrap()))
        };
        print_result(count, filename_str);
    } else if cli.chars {
        let count = if filename_str.is_empty() {
            count_chars(io::stdin().lock())
        } else {
            count_chars(BufReader::new(File::open(filename_str).unwrap()))
        };
        print_result(count, filename_str);
    }
}

fn print_result(count: usize, filename: &str) {
    if filename.is_empty() {
        println!("{}", count);
    } else {
        println!("{} {}", count, filename);
    }
}

fn count_bytes(mut reader: impl BufRead) -> usize {
    let mut buffer = [0; 8192];
    let mut total_bytes = 0;

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read the file");

        if bytes_read == 0 {
            break;
        }

        total_bytes += bytes_read;
    }

    total_bytes
}

fn count_lines(mut reader: impl BufRead) -> usize {
    let mut buffer = [0; 8192];
    let mut lines = 0;

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read");
        if bytes_read == 0 {
            break;
        }

        for &byte in &buffer[..bytes_read] {
            if byte == b'\n' {
                lines += 1;
            }
        }
    }
    lines
}

fn count_words(mut reader: impl BufRead) -> usize {
    let mut buffer = [0; 8192];

    let mut words = 0;
    let mut in_word = false;

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read");
        if bytes_read == 0 {
            break;
        }

        for &byte in &buffer[..bytes_read] {
            if byte.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                words += 1;
            }
        }
    }
    words
}

fn count_chars(mut reader: impl BufRead) -> usize {
    let mut chars_count = 0;

    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line).expect("Failed to read line");
        if bytes_read == 0 {
            break;
        }

        chars_count += line.chars().count();
    }
    chars_count
}

fn count_all(mut reader: impl BufRead) -> (usize, usize, usize) {
    let mut buffer = [0; 8192];
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut in_word = false;

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read");
        if bytes_read == 0 {
            break;
        }

        bytes += bytes_read;

        for &byte in &buffer[..bytes_read] {
            if byte == b'\n' {
                lines += 1;
            }

            if byte.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                words += 1;
            }
        }
    }
    (lines, words, bytes)
}
