use std::fs;

/// Print the `dir_path` dir.
fn print_dir(dir: &str) -> () {
  let it = fs::read_dir(dir).unwrap();

  for f in it {
    println!("{}", f.unwrap().file_name().into_string().unwrap());
  }
}

fn main() {
  print_dir(".");
}
