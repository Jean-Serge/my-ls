use std::fs;

fn main() {

  let it = fs::read_dir(".");

  if let Ok(files) = it {
    for file in files {
      if let Ok(f) = file {
        println!("{:?}", f.file_name());
      }
    }
  }
}
