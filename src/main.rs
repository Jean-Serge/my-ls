use std::fs;
use std::env;
use std::path::Path;

static path: &'static Path = Path::new(".");

/// Print the `dir_path` dir.
fn print_dir(dir: &str) -> () {
  // The unwrap function extracts the Ok() value of the Result
  let it = fs::read_dir(dir).unwrap();

  for f in it {
    // The into_string function will throw an error if it contains non-unicode characters
    println!("{}", f.unwrap().file_name().into_string().unwrap());
  }
}

fn parse_args() {
  let mut args = env::args();

  loop {
    match args.next() {
      Some(arg) => {
        path = Path::new(&arg);
      },
      None => break // There is no more argument
    }
  }
}

fn main() {
  parse_args();
//  print_dir(path);
}
