// write a FASTA parser
// test file

use std::io::{self, BufRead, Error};
use std::fs::File;
use std::path::Path;
use std::env;

fn main () {
    let args: Vec<String> = env::args().collect();

    let filename: &str = &args[1];
    let k: &usize = &args[2].parse::<usize>().unwrap();

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

    // count all possible k-mers in the sequence
    let mut kmer: Vec<String> = Vec::new();
    let mut count: Vec<usize> = Vec::new();
    for i in 0..seq.len() {
        let seq: String = seq[i].to_string();
        for j in 0..seq.len() - k + 1 {
            let mut kmer_tmp: String = String::new();
            for l in j..j + k {
                kmer_tmp.push(seq.chars().nth(l).unwrap());
            }
            if kmer.contains(&kmer_tmp) {
                let index: usize = kmer.iter().position(|x| *x == kmer_tmp).unwrap();
                count[index] += 1;
            }
            else {
                kmer.push(kmer_tmp);
                count.push(1);
            }
        }
    }

    // print k-mers
    for i in 0..kmer.len() {
        println!("{}: {}", kmer[i], count[i]);
    }

}