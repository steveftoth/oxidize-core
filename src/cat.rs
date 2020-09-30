use clap::{Arg, App, AppSettings};

fn main() {
  let matches = App::new("cat")
      .setting(AppSettings::TrailingVarArg)
      .version("0.1.0")
      .author("Steven Toth <steve@toths.info>")
      .about("Reimplementation of cat in rust")
      .arg(Arg::with_name("show-ends").short("E").long("show-ends").takes_value(false).help("display $ at end of each line"))
      .arg(Arg::with_name("xtra").index(1).multiple(true))
      .get_matches();

  let files: Vec<&str> = matches.values_of("xtra").unwrap().collect();
  println!("{}",files.join(",")
  );
}
