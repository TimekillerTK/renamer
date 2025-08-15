use crate::error::Result;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

// Define a struct to store file paths
pub struct FileEntry {
    pub path: PathBuf,
}

impl FileEntry {
    pub fn new(path: PathBuf) -> Self {
        FileEntry { path }
    }
}

pub struct FileEntries {
    pub entries: Vec<FileEntry>,
    pub dir_path: String,
}

impl FileEntries {
    pub fn new(dir_path: &str) -> Self {
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

        // Sort the file_entries alphanumerically
        file_entries.sort_by(|a, b| alphanumeric_sort::compare_path(&a.path, &b.path));

        Self {
            entries: file_entries,
            dir_path: dir_path.to_string(),
        }
    }

    pub fn rename(&self, name: &str, first_episode: usize, execute_rename: bool) -> Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
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
                first_episode + index,
                extension,
                width = self.zero_padding()
            );
            let new_path = old_path.with_file_name(&new_name);
            match execute_rename {
                true => {
                    // Rename the file
                    match fs::rename(old_path, &new_path) {
                        Ok(_) => {
                            println!("Renamed: {} -> {}", old_path.display(), new_path.display())
                        }
                        Err(e) => eprintln!("Error renaming {}: {}", old_path.display(), e),
                    }
                }
                false => {
                    println!("{} -> {}", old_path.display(), new_path.display());
                }
            }
        }
        Ok(())
    }

    pub fn add_zero_padding(&self, execute_rename: bool, which_number: usize) -> Result<()> {
        for entry in self.entries.iter() {
            let old_path = &entry.path;
            let re = Regex::new(r"\d+").unwrap();

            // Since file extensions can have numbers, we split the file stem from the file extension
            let mut file_stem = old_path.file_stem().unwrap().to_string_lossy().to_string();
            let file_extension = old_path.extension().unwrap().to_string_lossy().to_string();

            // Collect all matches' spans (which contain start & end range we can use later)
            let matches: Vec<_> = re.find_iter(&file_stem).collect();

            // Handle this later
            let selected_match = &matches[which_number - 1];

            // let first = &matches[0];
            let number_str = &file_stem[selected_match.range()];
            let padded_number = format!(
                "{:0width$}",
                number_str.parse::<u32>().unwrap(),
                width = &self.zero_padding()
            );

            file_stem.replace_range(selected_match.range(), &padded_number);
            let new_file_name = format!("{}.{}", file_stem, file_extension);
            let new_path = old_path.with_file_name(new_file_name);

            match execute_rename {
                true => {
                    // Rename the file
                    match fs::rename(old_path, &new_path) {
                        Ok(_) => {
                            println!("Renamed: {} -> {}", old_path.display(), new_path.display())
                        }
                        Err(e) => eprintln!("Error renaming {}: {}", old_path.display(), e),
                    }
                }
                false => {
                    println!("{} -> {}", old_path.display(), new_path.display());
                }
            };
        }
        Ok(())
    }

    /// Selects the number of leading zeroes needed for the number of files in the directory
    fn zero_padding(&self) -> usize {
        match self.entries.len() {
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
        }
    }
}
