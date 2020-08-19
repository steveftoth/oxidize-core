use clap::{Arg, App};

fn main() {
  let matches = App::new("cat")
      .version("0.1.0")
      .author("Steven Toth <steve@toths.info>")
      .about("Reimplementation of cat in rust")
      .arg(Arg::with_name("show-ends").short("E").long("show-ends").takes_value(false).help("display $ at end of each line"))
      .get_matches();


}
