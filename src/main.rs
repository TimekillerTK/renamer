use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// Define a struct to store file paths
struct FileEntry {
    path: PathBuf,
}

impl FileEntry {
    fn new(path: PathBuf) -> Self {
        FileEntry { path }
    }
}

fn main() {
    println!("\nType desired show name, for example (do not include episodes!):\n My Great Show - Season 1\n");
    io::stdout().flush().unwrap(); // to ensure the prompt is immediately displayed before waiting for input

    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line");

    let name = input_name.trim();

    // Specify the directory path
    let dir_path = "."; // Current directory

    // List files and create FileEntry structs
    let mut file_entries: Vec<FileEntry> = Vec::new();

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        file_entries.push(FileEntry::new(path));
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    // Sort the file_entries alphabetically
    file_entries.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));

    // Check how much padding we need depending on the number of files in a directory
    let padding = match file_entries.len() {
        0 => {
            todo!()
        }
        1..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        _ => {
            todo!()
        }
    };

    // Rename files
    println!("Renaming files in directory: {}", dir_path);
    for (index, entry) in file_entries.iter().enumerate() {
        let old_path = &entry.path;
        let extension = old_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");

        // Create the new file name
        let new_name = format!(
            "{} - E{:0width$}.{}",
            name,
            index + 1,
            extension,
            width = padding
        );
        let new_path = old_path.with_file_name(&new_name);

        println!("{} -> {}", old_path.display(), new_path.display());
    }

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

    // Rename files
    for (index, entry) in file_entries.iter().enumerate() {
        let old_path = &entry.path;

        let extension = old_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");

        // Create the new file name
        let new_name = format!(
            "{} - E{:0width$}.{}",
            name,
            index + 1,
            extension,
            width = padding
        );
        let new_path = old_path.with_file_name(&new_name);

        // Rename the file
        match fs::rename(old_path, &new_path) {
            Ok(_) => println!("Renamed: {} -> {}", old_path.display(), new_path.display()),
            Err(e) => eprintln!("Error renaming {}: {}", old_path.display(), e),
        }
    }
}
