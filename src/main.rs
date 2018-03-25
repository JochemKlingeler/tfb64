use std::io::Write;
use std::fs::File;
use std::io::prelude::*;

extern crate base64;
use base64::{encode};

fn main() {
    println!("Hello, world!");
    let mut arguments = Vec::new();

    for arg in std::env::args().skip(1) {
        arguments.push(arg);
    }
    if arguments.len() == 0 {
        writeln!(std::io::stderr(), "Usage: tfb64 FILEPATH FILEPATH").unwrap();
        std::process::exit(1); // exit out
    }
    // First arguement is file location
    let from_file_dir = arguments.get(0).expect("Oh noes");
    let mut f = File::open(from_file_dir).expect("File not found!");
    let mut content = Vec::new();
    if let Ok(meta) = f.metadata() {
        content.reserve(meta.len() as usize);
    }
    f.read_to_end(&mut content).expect("Unable to read file");
    let mut f_new = File::create(format!("{}{}", from_file_dir, ".base64")).expect("Could not write file");
    f_new.write(encode(&content).as_bytes()).expect("Unable to write into file");
}
