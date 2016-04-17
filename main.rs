use std::fs;

/// Print the `dir_path` dir.
fn print_dir(dir: &str) -> () {
  // The unwrap function extracts the Ok() value of the Result
  let it = fs::read_dir(dir).unwrap();

  for f in it {
    // The into_string function will throw an error if it contains non-unicode characters
    println!("{}", f.unwrap().file_name().into_string().unwrap());
  }
}

fn main() {
  print_dir(".");
}
