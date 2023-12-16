use std::fs::File;
use std::io::prelude::*;
extern crate chrono;
use chrono::*;

fn log_something(filename: &'static str, string: &[u8]) -> std::io::Result<()> {
    let mut f=(File::create(filename))?;
    (f.write_all(string));
    Ok(())
}
fn main() {
    let local:DateTime<Local> = Local::now();
    match log_something("log.txt", local) {
        Ok(..) => println!("Created and written"),
        Err(..) => println!("ERROR!")
    }
}