use std::fs;

fn main() {

  let it = fs::read_dir(".").unwrap();

  for f in it { 
    println!("{:?}", f.unwrap().file_name());
  };  
}
