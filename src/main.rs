use std::fs;
use std::env;

// Use String for not having to know the string size
struct Arguments {
  path: String
}

/// Print the `dir_path` dir.
fn print_dir(dir: &String) -> () {
  // The unwrap function extracts the Ok() value of the Result
  let it = fs::read_dir(dir).unwrap();

  for f in it {
    // The into_string function will throw an error if it contains non-unicode characters
    // TODO : Handle Error
    println!("{}", f.unwrap().file_name().into_string().unwrap());
  }
}

fn parse_args() -> Arguments {
  let mut args = env::args();
  let mut parsed_args = Arguments { path: String::from(".") };

  // Skip the 1st argument (executable name)
  args.next();

  // TODO : Check for Error (end of arguments)
  let path = args.next().unwrap();
  parsed_args.path = String::from(path.as_str());

  let final_args: Arguments = parsed_args;

  return final_args;
}

fn main() {
  // Gathering arguments
  let args = parse_args();

  // Passes reference to print_dir
  print_dir(&args.path);
}
