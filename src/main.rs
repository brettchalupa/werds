use clap::Parser;
use std::{io::stdin, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file(s) to read, use - to read from stdin (can be combined with files)
    files: Vec<PathBuf>,
    /// Print the number of lines in a file instead of words
    #[arg(short, long)]
    lines: bool,
}

struct WordyFile {
    path: PathBuf,
    word_count: usize,
    line_count: usize,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let mut wfiles: Vec<WordyFile> = Vec::new();

    for file in args.files {
        let mut wfile = WordyFile {
            word_count: 0,
            line_count: 0,
            path: file,
        };

        // Read from stdin if the specified file is `-`
        if wfile.path == PathBuf::from("-") {
            for line in stdin().lines() {
                wfile.word_count += words_in_line(line.unwrap());
                wfile.line_count += 1;
            }
            wfile.path = PathBuf::from("stdin");
        } else {
            match std::fs::metadata(&wfile.path) {
                Ok(md) => {
                    if md.is_dir() {
                        return handle_error(wfile.path, String::from("File is directory"));
                    }
                }
                Err(err) => return handle_error(wfile.path, err.to_string()),
            }

            let content = match std::fs::read_to_string(&wfile.path) {
                Ok(c) => c,
                Err(err) => return handle_error(wfile.path, err.to_string()),
            };

            for line in content.lines() {
                wfile.word_count += words_in_line(line.to_owned());
                wfile.line_count += 1;
            }
        }

        wfiles.push(wfile);
    }

    let mut summary: String = String::from("");

    if wfiles.len() > 1 {
        for wfile in wfiles.iter() {
            summary = format!(
                "{}{}: {}\n",
                summary,
                wfile.path.to_str().unwrap(),
                count_based_on_args(wfile, args.lines)
            );
        }

        summary = format!(
            "{}{}: {}",
            summary,
            "total",
            wfiles
                .iter()
                .fold(0, |r, wf| r + count_based_on_args(wf, args.lines))
        );
    } else {
        summary = format!("{}", count_based_on_args(&wfiles[0], args.lines));
    }

    println!("{}", summary);
    ExitCode::SUCCESS
}

fn words_in_line(line: String) -> usize {
    if line.trim().is_empty() {
        0
    } else {
        line.split(' ').count()
    }
}

fn count_based_on_args(wfile: &WordyFile, lines: bool) -> usize {
    if lines {
        wfile.line_count
    } else {
        wfile.word_count
    }
}

fn handle_error(file: PathBuf, error_message: String) -> ExitCode {
    eprintln!("Error! {}: {}", error_message, file.to_str().unwrap());
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{count_based_on_args, words_in_line};

    #[test]
    fn words_in_line_properly_counts() {
        assert_eq!(words_in_line(String::from("hi")), 1);
        assert_eq!(words_in_line(String::from("hi there")), 2);
        assert_eq!(words_in_line(String::from("")), 0);
        assert_eq!(words_in_line(String::from("I love counting words!")), 4);
    }

    #[test]
    fn count_based_on_args_keys_off_lines_bool() {
        let wfile = crate::WordyFile {
            path: PathBuf::from("example.txt"),
            word_count: 10,
            line_count: 2,
        };
        assert_eq!(count_based_on_args(&wfile, false), 10);
        assert_eq!(count_based_on_args(&wfile, true), 2);
    }
}
