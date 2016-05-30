use std::fs;
use std::env;

// Use String for not having to know the string size
pub struct Arguments {
  pub path: String
}

/// Print the `dir_path` dir.
pub fn print_dir(dir: &String) -> () {
  // The unwrap function extracts the Ok() value of the Result
  let mut it = fs::read_dir(dir).unwrap();

  loop {
    match it.next() {
      Some(file) => match file {
          Ok(f)  => println!("{}", f.file_name().into_string().unwrap()),
          Err(e) => println!("{}", e)
      },
      None => break // No more entries
    }
  }
}

/// Parse all command line arguments
/// For now, it only look for a path in argument list
pub fn parse_args() -> Arguments {
  let mut args = env::args();
  let mut parsed_args = Arguments { path: String::from(".") };

  // Skip the first argument (executable name)
  args.next();

  loop {
    match args.next() {
      Some(arg) => parsed_args.path = String::from(arg.as_str()),
      None      => break // No more arguments
    }
  }

  return parsed_args;
}
