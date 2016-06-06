use std::fs;

/// Print the `dir_path` dir.
pub fn print_dir(dir: &String) -> () {
    // The unwrap function extracts the Ok() value of the Result
    let mut it = fs::read_dir(dir).unwrap();

    println!("Listing {}", dir);

    loop {
        match it.next() {
            Some(file) => {
                match file {
                    Ok(f) => println!("  {}", f.file_name().into_string().unwrap()),
                    Err(e) => println!("{}", e),
                }
            }
            None => break, // No more entries
        }
    }
}
