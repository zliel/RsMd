use std::{fs::File, io::Read};

pub fn read_file(file_path: &str) -> String {
    let mut md_file: File = File::open(file_path).expect("Couldn't open file");
    let mut contents = String::new();
    md_file
        .read_to_string(&mut contents)
        .expect("Couldn't read file into string");

    contents
}
