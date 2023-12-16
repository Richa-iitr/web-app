#[macro_use] extern crate nickel;
use nickel::Nickel;
extern crate clap;
use clap::{Arg,Command};
extern crate chrono;
use chrono::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

fn get_dateTime(filename:&String) -> io::Result<()> {
    let mut f = OpenOptions::new().append(true).create(true).open(filename)?;
    let local: DateTime<Local> = Local::now();
    let local_str = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let time = local_str.as_bytes();
    f.write_all(time);
    Ok(())
}
fn log_dateTime(filename:&String) -> String {
    match get_dateTime(filename) {
        Ok(..) => format!("Created and logged"),
        Err(..) => format!("Error encountered")
    }
}

fn main() {
    let mut server = Nickel::new();
    let matches = Command::new("My App").version("0.1.0").author("iri")
            .arg(Arg::new("filename").short('f').required(true).help("type the filename to log to"))
            .arg(Arg::new("auth-token").short('a').help("Auth token"))
            .get_matches();
    let default = "log_test.txt".to_string();
    let myfile = matches.get_one::<String>("filename").unwrap_or(&default).to_string();
    let auth = match matches.get_one::<String>("auth-token"){
        Some(str) => str.to_string(),
        None => "".to_string()
    };
    println!("{} auth", auth);
    server.utilize(router! {
        get "**" => |_req,_res| {
            log_dateTime(&myfile)
        }
    });
    //listen on server at localhost:6767
    server.listen("127.0.0.1:6767");

    //to run: cargo run
    //to test: curl http://localhost:6767
}
