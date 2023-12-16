use std::fs::{File,OpenOptions};
use std::io::prelude::*;
extern crate chrono;
use chrono::*;

fn log_something(filename: &'static str) -> std::io::Result<()> {
    let mut f=OpenOptions::new().append(true).create(true).open(filename)?;
    let local:DateTime<Local> = Local::now();
    let time_str = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let str = time_str.as_bytes();
    (f.write_all(str));
    Ok(())
}
fn main() {
    match log_something("log.txt") {
        Ok(..) => println!("Created and written"),
        Err(..) => println!("ERROR!")
    }
}