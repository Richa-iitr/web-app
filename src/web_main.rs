#[macro_use] extern crate nickel;
use nickel::Nickel;

fn print_hello() -> &'static str {
    "hello world!"
}

fn main() {
    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req,_res| {
            print_hello()
        }
    });
    //listen on server at localhost:6767
    server.listen("127.0.0.1:6767");

    //to run: cargo run
    //to test: curl http://localhost:6767
}
