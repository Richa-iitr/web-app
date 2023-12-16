use std::fs::File;
use std::io::prelude::*;

fn log_something(filename: &'static str, string: &[u8]) -> std::io::Result<()> {
    let mut f=(File::create(filename))?;
    (f.write_all(string));
    Ok(())
}
fn main() {
    match log_something("log.txt", b"Alive") {
        Ok(..) => println!("Created and written"),
        Err(..) => println!("ERROR!")
    }
}