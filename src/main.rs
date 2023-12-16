extern crate clap;
use clap::{Arg,Command};

//run file crago run -- -f test.txt
fn main() {
    let matches = Command::new("My App").version("0.1.0").author("iri").arg(Arg::new("filename").short('f').help("type the filename to log to")).get_matches();
    let default = "log_test.txt".to_string();
    let myfile = matches.get_one::<String>("filename").unwrap_or(&default);
    println!("{} file", myfile);
}