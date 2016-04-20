use std::fs;
use std::env;
use std::path::Path;

struct Arguments<'a> {
  path: &'a str
}

/// Print the `dir_path` dir.
fn print_dir(dir: &str) -> () {
  // The unwrap function extracts the Ok() value of the Result
  let it = fs::read_dir(dir).unwrap();

  for f in it {
    // The into_string function will throw an error if it contains non-unicode characters
    println!("{}", f.unwrap().file_name().into_string().unwrap());
  }
}

unsafe fn parse_args() -> Arguments<'static> {
  let mut args = env::args();
  let mut parsed_args = Arguments { path: "." };

    match args.next() {
      Some(arg) => {
        let path: &'static str = arg.as_str();
        parsed_args.path = path;
      },
      None => println!("Arguments parsing done !")// There is no more argument
    }

  let final_args: Arguments = parsed_args;
  return final_args;
}

fn main() {
  unsafe {
    parse_args();
  }
//  print_dir(path);
}
