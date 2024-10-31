use std::io::{self, Write};
pub mod cli;
pub mod structs;
use crate::cli::Cli;
use clap::Parser;
use structs::FileEntries;

fn main() {
    // CLI
    let cli = Cli::parse();

    let path = match cli.path {
        Some(path) => path,
        None => {
            String::from(".") // Default to the current directory if no path is provided
        }
    };

    println!("\nType desired show name, for example (do not include episodes!):\n My Great Show - Season 1\n");
    io::stdout().flush().unwrap(); // to ensure the prompt is immediately displayed before waiting for input

    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line");

    let name = input_name.trim();

    let file_entries = FileEntries::new(&path);

    // Rename files
    println!("Renaming files in directory: {}", file_entries.dir_path);
    file_entries.rename(name, false);

    println!("\nIs this OK? Type 'OK' to continue and rename the files as shown.");
    io::stdout().flush().unwrap(); // to ensure the prompt is immediately displayed before waiting for input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();
    if trimmed_input == "OK" {
        println!("OK, proceeding...");
        // Execute the desired code for the "yes" case
    } else {
        println!("Exiting the program.");
        return;
    }

    file_entries.rename(name, true);
}
