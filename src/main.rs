use std::io::{self, Write};
pub mod cli;
pub mod error;
pub mod structs;
use crate::cli::Cli;
use crate::error::Result;
use clap::Parser;
use structs::FileEntries;

fn main() -> Result<()> {
    // CLI
    let cli = Cli::parse();

    let file_entries = FileEntries::new(&cli.path);

    // Check whether we are using the main renaming function or just doing zero padding
    let first_episode_name = if let true = cli.zero_padding {
        println!("Renaming files in directory: {}", file_entries.dir_path);
        file_entries.add_zero_padding(false, cli.number)?;

        None
    } else {
        println!("\nType desired show name, for example (do not include episodes!):\n My Great Show - Season 1\n");
        io::stdout().flush().unwrap(); // to ensure the prompt is immediately displayed before waiting for input

        let mut input_name = String::new();
        io::stdin()
            .read_line(&mut input_name)
            .expect("Failed to read line");

        let name = input_name.trim().to_string();

        println!("\nWhich is the first episode? Press ENTER to start from 1.\n");
        io::stdout().flush().unwrap(); // to ensure the prompt is immediately displayed before waiting for input
        let mut input_episode_number = String::new();
        io::stdin()
            .read_line(&mut input_episode_number)
            .expect("Failed to read line");

        let first_episode = match input_episode_number.trim().parse::<usize>() {
            Ok(x) => x,
            Err(_) => 1,
        };

        // Rename files
        println!("Renaming files in directory: {}", file_entries.dir_path);
        file_entries.rename(&name, first_episode, false)?;

        Some((first_episode, name))
    };

    print!("\nIs this OK? Type 'OK' to continue and rename the files as shown.");
    if cli.zero_padding {
        println!(" TIP: type --number to select the number which should be modified (first one by default).");
    };
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
        return Ok(());
    }

    // Actually perform the operations
    match first_episode_name {
        Some((first_episode, name)) => file_entries.rename(&name, first_episode, true),
        None => file_entries.add_zero_padding(true, cli.number),
    }
}
