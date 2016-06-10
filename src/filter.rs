use std::fs::DirEntry;

use std::io::Error;

pub fn no_hidden(entry: &Result<DirEntry, Error>) -> bool {
    match *entry {
        Ok(ref f) => !f.file_name().to_str().unwrap().starts_with("."),
        _ => false,
    }
}

pub fn no_filter(_: &Result<DirEntry, Error>) -> bool {
    true
}
