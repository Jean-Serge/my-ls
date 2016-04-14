use std::fs;

fn is_directory(elt : std::io::Result<std::fs::FileType>) ->  bool {
    match elt {
        Err(_) => false,
        Ok(d)  => d.is_dir()
    }
}

fn to_str(osstr : std::ffi::OsString) -> String {
    match osstr.into_string() {
        Err(_) => String::from("????"),
        Ok(st) => st
    }
}


fn print_file_name(elt : std::io::Result<std::fs::DirEntry>) {
    match elt {
        Err(_) => println!("unknown error for this file"),
        Ok(file) => {
            let emoji = if is_directory(file.file_type()) {"📁"} else {"📝"} ;
            println!("{} {}", emoji, to_str(file.file_name()))
        }
    }
}

fn main() {
    match fs::read_dir(".") {
        Err(msg) => println!("{}", msg),
        Ok(it)   =>
            for entry in it {
                print_file_name(entry)
            }
    }
}


