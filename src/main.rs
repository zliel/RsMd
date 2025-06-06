mod io;

use io::read_file;
use std::env::args;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = args().collect();
    let file_path = &args[1];
}
