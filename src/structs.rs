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

        // Sort the file_entries alphabetically
        file_entries.sort_by(|a, b| a.path.file_name().cmp(&b.path.file_name()));

        Self {
            entries: file_entries,
            dir_path: dir_path.to_string(),
        }
    }

    pub fn rename(&self, name: &str, first_episode: usize, execute_rename: bool) -> Option<()> {
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
        Some(())
    }

    /// Selects the number of padding needed for the number of files in the directory
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
