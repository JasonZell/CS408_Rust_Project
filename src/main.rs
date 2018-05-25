/*
 Document Analyzer which takes the input of a text document and outputs a detailed summary of the document.
The summary report includes: word count, paragraph count, sentence count, top ten most frequently used words, and the number of words that
begin with each alphabets.
*/

#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {

    println!("This is a prototype 1.0");

    let delimiter: &[_] = &['!','.',',','?'];
    let mut paragraph_count = 0;
    let mut sentence_count = 0;
    let mut word_count = 0;
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
        println!("line: {}",&s);

        { // new scope necessary for borrowing s into iter
            let iter = s.split_whitespace();
            //let counter_iter = s.split_whitespace();
            let mut count = 0;

            for str in iter
                {
                    word_count = word_count + 1;
                    count = count + 1;
                    println!("str: {}", &str,);
                    if str.find(|c : char| (c == '.') | (c == '!') | (c == '?') )!= None  // mark the end of sentence
                        {
                            sentence_count =  sentence_count + 1;
                        }
                    let good_str = str.trim_matches(delimiter);
                    // println!("goodstr: {}",good_str);
                }

            if count == 0
                {
                    println!("-----NEW PARAGRAPH------");
                    paragraph_count = paragraph_count + 1;
                }

        }

        // this returns the ownership of the read data to buf
        // there is no allocation

        buf = s.into_bytes();
        buf.clear();
    }
    println!("Word Count: {}",word_count);
    println!("Sentence Count: {}",sentence_count);
    println!("Paragraph Count: {}",paragraph_count);
}





