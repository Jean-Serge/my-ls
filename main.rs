use std::fs;

fn main() {

  let it = fs::read_dir(".").unwrap();

  let x = it.flat_map(|file| {
    file.unwrap().file_name();
  }).collect();

  println!("{:?}", x);

  for f in it { 
    println!("{:?}", f.unwrap().file_name());
  };
}
