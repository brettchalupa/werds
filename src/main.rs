use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the file(s) to read
    files: Vec<String>
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let mut summary:String = String::from("");
    let mut total_word_count = 0;
    let file_count = args.files.len();

    for file in args.files {
        let content;
        match std::fs::read_to_string(&file) {
            Ok(c) => { content = c },
            Err(err) => { return handle_error(file, err) }
        }
        let mut file_word_count = 0;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue
            }
            let count = line.split(' ').count();
            total_word_count += count;
            file_word_count += count;
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

fn handle_error(file:String, error:std::io::Error) -> ExitCode {
    eprintln!("Error! {}: {}", error.to_string(), file);
    ExitCode::FAILURE
}
