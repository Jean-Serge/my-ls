use std::fs;
use std::fs::DirEntry;

use std::iter;

pub fn filter_hidden(file: DirEntry) -> bool {
    match file.file_name().to_str() {
        Some(n) => !n.starts_with("."),
        None    => false,
    }
}
