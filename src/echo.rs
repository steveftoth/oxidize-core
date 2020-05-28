use std::env;

fn main() {
  let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
  println!("{}", args);

}


