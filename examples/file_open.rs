use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {
    let filename = PathBuf::from("/Users/naterichman/temp.txt");
    match File::create(&filename) {
        Ok(f) => println!("{:?}", f),
        Err(e) => println!("{:?}", e.to_string()),
    }
}
