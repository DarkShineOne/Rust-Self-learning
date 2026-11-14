use std::path::PathBuf;

use basics_15_console::{JsonFileRepository, run_cli};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = std::env::var_os("FEEDBACK_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("feedback.json"));
    let repository = JsonFileRepository::new(path);

    match run_cli(&args, repository) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
