// write a FASTA parser
// test file

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;


fn parse_fasta (filename: &str) -> (Vec<String>, Vec<String>) {
    let path: &Path = Path::new(filename);
    let display: std::path::Display<'_> = path.display();
    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut headers: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.starts_with('>') {
            let header: String = line[1..].to_string();
            if headers.contains(&header) {
                println!("Warning: duplicate header {} in file {}. Skipping...", line, filename);
            }
            else {
                headers.push(header);
            }
        }
        else {
            seq.push(line);
        }
    }
    return (headers, seq)
}

fn count_kmers (seq: &str, k: usize) -> (Vec<String>, Vec<usize>) {
    let mut kmer: Vec<String> = Vec::new();
    let mut count: Vec<usize> = Vec::new();
    for i in 0..seq.len() - k + 1 {
        let mut kmer_tmp: String = String::new();
        for j in i..i + k {
            kmer_tmp.push(seq.chars().nth(j).unwrap());
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
    return (kmer, count)
}

fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <filename> <k-mer length>", args[0]);
        std::process::exit(1);
    }

    let filename: &str = &args[1];
    let k: &usize = &args[2].parse::<usize>().unwrap();

    // read FASTA file
    let (headers, seq): (Vec<String>, Vec<String>) = parse_fasta(filename);

    // iterate over sequence and count kmers
    let mut kmer: Vec<String> = Vec::new();
    let mut count: Vec<usize> = Vec::new();
    for i in 0..seq.len() {
        let (kmer_tmp, count_tmp): (Vec<String>, Vec<usize>) = count_kmers(&seq[i], *k);
        for j in 0..kmer_tmp.len() {
            if kmer.contains(&kmer_tmp[j]) {
                let index: usize = kmer.iter().position(|x| *x == kmer_tmp[j]).unwrap();
                count[index] += count_tmp[j];
            }
            else {
                kmer.push(kmer_tmp[j].clone());
                count.push(count_tmp[j]);
            }
        }
    }

    // print k-mers
    for i in 0..kmer.len() {
        println!("{}\t{}", kmer[i], count[i]);
    }

}