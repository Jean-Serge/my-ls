use std::fs;
use std::io;

/// Print the `dir_path` dir.
pub fn print_dir<F>(path: &String, filter: F) -> ()
    where F : Fn(&Result<fs::DirEntry,io::Error>) -> bool {
    let it = fs::read_dir(path).unwrap();
    let mut filtered = it.filter(filter);

    loop {
        match filtered.next() {
            Some(file) => {
                match file {
                    Ok(f) => println!("{}", f.file_name().into_string().unwrap()),
                    Err(e) => println!("{}", e),
                }
            }
            None => break, // No more entries
        }
    }
}
