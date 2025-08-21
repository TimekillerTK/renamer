use clap::{arg, command, Parser};

/// Simple renamer program.
///
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    long_about = "
Simple renamer program. Safe to use, you must confirm with 'OK' for any rename to actually happen.

EXAMPLES:
  renamer
    └-- Renames files in current directory. File order will be preserved alphanumerically.

  renamer --path /path/to/folder
    └-- Renames files in specified directory

  renamer --zero-padding
    └-- Adds leading zeroes to first number of the file all files in current directory

  renamer --zero-padding --path /path/to/folder
    └-- Adds leading zeroes to first number of the file for all files in specified directory

  renamer --zero-padding --number 2
    └-- Adds leading zeroes to the second number of the file for all files in current directory
"
)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to the directory used
    #[arg(long, short, default_value_t = String::from("."))]
    pub path: String,
    /// Adds leading zeroes to a number in all files in directory based on the number of files. So, if there are 101 total files, the files will be changed 1 -> 001, 2 -> 002, ..., 10 -> 010, ... 99 -> 099, 100 -> 100. To select which number gets modified, use --number
    #[arg(long, short)]
    pub zero_padding: bool,
    /// Selects the number in the file name which should have added leading zeroes (only used with --zero-padding)
    #[arg(long, short, default_value_t = 1)]
    pub number: usize,
}
