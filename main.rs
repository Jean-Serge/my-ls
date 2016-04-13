use std::fs;

fn main() {


 // let it = match fs::read_dir(".") {
 //   Ok(v) => v,
 //   Err(e) => ()
 // };

  let it = fs::read_dir(".");
  if let Err(e) = it {
    println!("{:?}", e);
  }

  println!("{:?}", it.count());

}
