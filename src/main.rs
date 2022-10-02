use clap::Parser;
use std::{io::stdin, process::ExitCode};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file(s) to read, use - to read from stdin (can be combined with files)
    files: Vec<String>,
}

fn words_in_line(line: String) -> usize {
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

    let mut summary: String = String::from("");
    let mut total_word_count = 0;
    let file_count = args.files.len();

    for file in args.files {
        let mut file_word_count = 0;
        let mut file = file;

        // Read from stdin if the specified file is `-`
        if file == "-" {
            for line in stdin().lines() {
                let count = words_in_line(line.unwrap());
                file_word_count += count;
                total_word_count += count;
            }
            file = String::from("stdin");
        } else {
            let content;

            match std::fs::metadata(&file) {
                Ok(md) => {
                    if md.is_dir() {
                        return handle_error(file, String::from("File is directory"));
                    }
                }
                Err(err) => return handle_error(file, err.to_string()),
            }

            match std::fs::read_to_string(&file) {
                Ok(c) => content = c,
                Err(err) => return handle_error(file, err.to_string()),
            }

            for line in content.lines() {
                let count = words_in_line(line.to_owned());
                total_word_count += count;
                file_word_count += count;
            }
        }

        if file_count > 1 {
            summary = format!("{}{}: {}\n", summary, file, file_word_count);
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

fn handle_error(file: String, error_message: String) -> ExitCode {
    eprintln!("Error! {}: {}", error_message, file);
    ExitCode::FAILURE
}
