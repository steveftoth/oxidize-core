use std::fs::File;
use clap::{Arg, App, AppSettings};

fn read_files(paths: Vec<&str>) -> Result<String, io::Error>{
println!("{}" , paths.join(" "));
for p in paths {
    let f = File::open(p)?;
    let mut 
}

}

fn main()  -> std::io::Result<()>{
  let matches = App::new("cat")
      .setting(AppSettings::TrailingVarArg)
      .version("0.1.0")
      .author("Steven Toth <steve@toths.info>")
      .about("Reimplementation of cat in rust")
      .arg(Arg::with_name("show-ends").short("E").long("show-ends").takes_value(false).help("display $ at end of each line"))
      .arg(Arg::with_name("xtra").index(1).multiple(true))
      .get_matches();

    match matches.values_of("xtra") {
        Some(v) => read_files(v.collect()),
            None => println!("No files to read"),
    }
}
