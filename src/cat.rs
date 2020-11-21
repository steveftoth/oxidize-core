use clap::{App, AppSettings, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_files(paths: Vec<&str>) -> Result<(), std::io::Error> {
    println!("{}", paths.join(" "));
    for p in paths {
        let mut f = match File::open(&Path::new(p)) {
            Err(why) => return Err(why),
            Ok(f) => f,
        };

        let mut s = String::new();
        let mut buffer = [0; 10];
        match f.read_to_string(&mut s) {
            Err(why) => return Err(why),
            Ok(_) => print!("{}", s),
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("cat")
        .setting(AppSettings::TrailingVarArg)
        .version("0.1.0")
        .author("Steven Toth <steve@toths.info>")
        .about("Reimplementation of cat in rust")
        .arg(
            Arg::with_name("show-ends")
                .short("E")
                .long("show-ends")
                .takes_value(false)
                .help("display $ at end of each line"),
        )
        .arg(Arg::with_name("xtra").index(1).multiple(true))
        .get_matches();
    match matches.values_of("xtra") {
        Some(v) => read_files(v.collect()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No files to read",
        )),
    }
}
