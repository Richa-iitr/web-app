#[macro_use] extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

fn get_dateTime(filename:&'static str) -> io::Result<()> {
    let mut f = OpenOptions::new().append(true).create(true).open(filename)?;
    let local: DateTime<Local> = Local::now();
    let local_str = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let time = local_str.as_bytes();
    f.write_all(time);
    Ok(())
}
fn log_dateTime(filename:&'static str) -> String {
    match get_dateTime(filename) {
        Ok(..) => format!("Created and logged"),
        Err(..) => format!("Error encountered")
    }
}

fn main() {
    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req,_res| {
            log_dateTime("log.txt")
        }
    });
    //listen on server at localhost:6767
    server.listen("127.0.0.1:6767");

    //to run: cargo run
    //to test: curl http://localhost:6767
}
