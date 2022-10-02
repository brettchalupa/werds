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

struct WordyFile {
    path: String,
    word_count: usize,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let mut wfiles: Vec<WordyFile> = Vec::new();

    for file in args.files {
        let mut wfile = WordyFile {
            word_count: 0,
            path: file.to_string(),
        };

        // Read from stdin if the specified file is `-`
        if wfile.path == "-" {
            for line in stdin().lines() {
                wfile.word_count += words_in_line(line.unwrap());
            }
            wfile.path = String::from("stdin");
        } else {
            match std::fs::metadata(&wfile.path) {
                Ok(md) => {
                    if md.is_dir() {
                        return handle_error(wfile.path, String::from("File is directory"));
                    }
                }
                Err(err) => return handle_error(wfile.path, err.to_string()),
            }

            let content;

            match std::fs::read_to_string(&wfile.path) {
                Ok(c) => content = c,
                Err(err) => return handle_error(wfile.path, err.to_string()),
            }

            for line in content.lines() {
                wfile.word_count += words_in_line(line.to_owned());
            }
        }

        wfiles.push(wfile);
    }

    let mut summary: String = String::from("");

    if wfiles.len() > 1 {
        for wfile in wfiles.iter() {
            summary = format!("{}{}: {}\n", summary, wfile.path, wfile.word_count);
        }

        summary = format!(
            "{}{}: {}",
            summary,
            "total",
            wfiles.iter().fold(0, |r, s| r + s.word_count)
        );
    } else {
        summary = format!("{}", wfiles[0].word_count);
    }

    println!("{}", summary);
    ExitCode::SUCCESS
}

fn handle_error(file: String, error_message: String) -> ExitCode {
    eprintln!("Error! {}: {}", error_message, file);
    ExitCode::FAILURE
}
