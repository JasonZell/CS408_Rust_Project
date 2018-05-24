use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("This is a prototype 1.0");

    // let mut lineNumber;
    // let mut ParaNumber;
    // let mut SentNumber;

    let args: Vec<String> = env::args().collect();

    if args.len()< 2
        {
            println!("Error! Must pass in file name as argument.");
            return;
        }
    let filename = &args[1];

    // let mut f = File::open(filename).expect("file not found");
    let mut f = BufReader::new(File::open(filename).expect("open failed"));

    let mut buf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("Character: {}", c);
        }
        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }

}



