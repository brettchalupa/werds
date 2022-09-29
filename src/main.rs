use clap::Parser;
use std::{process::ExitCode, io::stdin};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the file(s) to read
    files: Vec<String>
}

fn words_in_line(line:String) -> usize {
    if line.trim().is_empty() {
        0
    } else {
        line.split(' ').count()
    }
}

#[cfg(test)]
mod tests {
    use crate::words_in_line;

    #[test]
    fn words_in_line_properly_counts() {
        assert_eq!(words_in_line(String::from("hi")), 1);
        assert_eq!(words_in_line(String::from("hi there")), 2);
        assert_eq!(words_in_line(String::from("")), 0);
        assert_eq!(words_in_line(String::from("I love counting words!")), 4);
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let mut summary:String = String::from("");
    let mut total_word_count = 0;
    let file_count = args.files.len();

    if args.files.is_empty() {
        for line in stdin().lines() {
            total_word_count += words_in_line(line.unwrap());
        }
    } else {
        for file in args.files {
            let content;
            match std::fs::read_to_string(&file) {
                Ok(c) => { content = c },
                Err(err) => { return handle_error(file, err) }
            }
            let mut file_word_count = 0;

            for line in content.lines() {
                let count = words_in_line(line.to_owned());
                total_word_count += count;
                file_word_count += count;
            }

            if file_count > 1 {
                summary = format!("{}{}: {}\n", summary, file, file_word_count);
            }
        }
    }

    if file_count > 1 {
        summary = format!("{}{}: {}", summary, "total", total_word_count);
    } else {
        summary = format!("{}", total_word_count);
    }

    println!("{}", summary);
    ExitCode::SUCCESS
}

fn handle_error(file:String, error:std::io::Error) -> ExitCode {
    eprintln!("Error! {}: {}", error.to_string(), file);
    ExitCode::FAILURE
}
