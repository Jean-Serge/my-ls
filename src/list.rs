use std::fs;
use std::io;

enum Color {
    BLUE,
    GREEN,
    RED,
    DEFAULT
}

fn color_to_bytes<'a>(c: Color) -> &'a str {
    match c {
        Color::BLUE => "\x1B[34m",
        Color::GREEN => "\x1B[32m",
        Color::RED => "\x1B[31m",
        Color::DEFAULT => "\x1B[39m"
    }
}

fn color_of_type<'a>(t: fs::FileType) -> &'a str {
    if t.is_dir() {
        color_to_bytes(Color::BLUE)
    } else if t.is_file() {
        color_to_bytes(Color::DEFAULT)
    } else {
        color_to_bytes(Color::GREEN)
    }
}

/// Print the `dir_path` dir.
pub fn print_dir<F>(path: &String, filter: F) -> ()
    where F: Fn(&Result<fs::DirEntry, io::Error>) -> bool
{
    let it = fs::read_dir(path).unwrap();
    let mut filtered = it.filter(filter);

    loop {
        match filtered.next() {
            Some(file) => {
                match file {
                    Ok(f) => print_file(f),
                    Err(e) => println!("{}", e),
                }
            }
            None => break, // No more entries
        }
    }
}

/// Print a specific file
fn print_file(f: fs::DirEntry) -> () {
    let default = color_to_bytes(Color::DEFAULT);
    let file_name = f.file_name().into_string().unwrap();
    let color = match f.file_type() {
        Ok(t) => color_of_type(t),
        Err(_) => color_to_bytes(Color::RED)
    };

    println!("{}{}{}", color, file_name, default);
}
