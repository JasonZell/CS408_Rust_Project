/*

CS40801- Sp18
Team 5 Rust Project

Description:
This program is a Document Analyzer which takes the input of a text document and outputs a detailed summary of the document.
The summary report includes: word count, paragraph count, sentence count, top ten most frequently used words, and the number of words that
begin with each alphabets (A-Z).

How to build:
    1. Must install Rust Compiler ( https://www.rust-lang.org/en-US/install.html ), available for Windows, MacOS, Linux and many others.(https://forge.rust-lang.org/platform-support.html)
    2. Open command prompt and go to the root source directory that has Cargo.toml file.
    3. To build binary with debug profile, run 'cargo build' command, the binary will be build into the target folder.
    4. Alternatively, run 'cargo build --release' to build with release profile.

How To Run:
    1. open command prompt and go to the directory with the binary executable.
    2. Put the text document in the same folder.
    3. run the executable while pass in the document file name as the only argument.
    4. e.g. in Windows: programBinary.exe fileName.txt

*/

#![allow(unused)]
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]
struct Heappair {
    word : String,
    count : u32,
}
impl Ord for Heappair {
    fn cmp(&self, other: &Heappair) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Heappair {
    fn partial_cmp(&self, other: &Heappair) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Heappair {
    fn eq(&self, other: &Heappair) -> bool {
        self.count == other.count
    }
}

fn main() {
    println!("Version 1.1");

    let delimiter: &[_] = &['!','.',',','?'];
    let alpha_array : [char;26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N',
        'O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let args: Vec<String> = env::args().collect();
    let mut paragraph_count = 0;
    let mut sentence_count = 0;
    let mut word_count = 0;
    let mut word_frequency : HashMap<String, u32> = HashMap::new();
    let mut heap : BinaryHeap<Heappair> = BinaryHeap::new();
    let mut alpha_frequency : HashMap<char,u32> = HashMap::new();

    if args.len()< 2
        {
            println!("Error! Must pass in file name as argument.");
            return;
        }
    println!("Parsing... Please be patient...");

    let filename = &args[1];
    let mut f = BufReader::new(File::open(filename).expect("open failed"));
    let mut buf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {

        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("from_utf8 failed");

        { // new scope necessary for borrowing s into iter
            let iter = s.split_whitespace();
            let mut count = 0;

            for str in iter
                {
                    word_count = word_count + 1;
                    count = count + 1;
                    if str.find(|c : char| (c == '.') | (c == '!') | (c == '?') )!= None  // mark the end of sentence
                        {
                            sentence_count =  sentence_count + 1;
                        }
                    let good_str = str.trim_matches(delimiter);
                    let first_char = good_str.chars().next();
                    match first_char {
                        Some(c) =>
                            {
                                // entry function will try to get the value base on key, if key doesn't exist, it will insert the key with value of 1
                                let count =  alpha_frequency.entry(c.to_ascii_uppercase()).or_insert(1);
                                *count += 1;
                            }
                        None => {},
                    }
                    if word_frequency.get(good_str) == None
                        {
                            word_frequency.insert(good_str.to_string(),1);
                        }
                        else {
                            let counter =  word_frequency.entry(good_str.to_string()).or_insert(0);
                            *counter += 1;
                        }
                }

            if count == 0
                {
                    paragraph_count = paragraph_count + 1;
                }
        } // iter will be dropped here. so it won't outlive its referent 's'

        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }

    for(k,v) in word_frequency.drain()
        {
            let pair = Heappair{word : k.to_string(), count : v};
            heap.push(pair);
        }
    let mut pair : Heappair;
    let len = heap.len();

    println!("\nSummary:\n");
    println!("Word Count: {}",word_count);
    println!("Sentence Count: {}",sentence_count);
    println!("Paragraph Count: {}",paragraph_count);

    println!("\nTop Ten Most Frequently Used Words and their counts:");
    if len >= 10
        {
            for x in 0..10
                {
                    pair = heap.pop().unwrap();

                    println!("{:5}: {}",pair.word, pair.count);
                }
        }
        else {
            for x in 0..len
                {
                    pair = heap.pop().unwrap();

                    println!("{:5}: {}",pair.word, pair.count);
                }

        }
    println!("\nNumber of words that starts with:");
    for i in 0..26 {
        println!("{} : {} ",alpha_array[i],alpha_frequency.get(&alpha_array[i]).unwrap());

    }
}





