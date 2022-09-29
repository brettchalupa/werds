use clap::Parser;
use std::error::Error;

/// Displays the number of words in the specified files. Inspired by `wc`.
#[derive(Parser)]
struct Cli {
    /// The path to the file(s) to read
    files: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut summary:String = String::from("");
    let mut total_word_count = 0;
    let file_count = args.files.len();

    for file in args.files {
        let content = std::fs::read_to_string(&file)?;
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
    Ok(())
}
