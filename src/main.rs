// write a FASTA parser
// test file

use std::io::{self, BufRead, Error};
use std::fs::File;
use std::path::Path;
use core::iter::Zip;

fn main () {
    let filename: &str = "test.fa";
    let path: &Path = Path::new(filename);
    let display: std::path::Display<'_> = path.display();
    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.starts_with('>') {
            header.push(line);
        }
        else {
            seq.push(line);
        }
    }

    // print interleaved
    let iter = header.iter().zip(seq.iter());
    for (h, s) in iter {
        println!("{}\n{}", h, s);
    }

}