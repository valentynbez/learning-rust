// write a FASTA parser
// test file

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;


struct Sequence {
    header: String,
    seq: String,
    kmers: HashMap<String, usize>
}



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
    let mut skip_entry: bool = false;
    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.starts_with('>') {
            let header: String = line[1..].to_string();
            if headers.contains(&header) {
                println!("Warning: duplicate header {} in file {}. Skipping...", line, filename);
                skip_entry = true;
            }
            else {
                headers.push(header);
                skip_entry = false;
            }
        }
        else if !skip_entry{
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
    for s in &seq {
        println!("{}", s);
    }

    // iterate over sequence and count kmers in each row
    let mut seqs_kmers: Vec<Sequence> = Vec::new();
    for i in 0..seq.len() {
        let mut kmer_count = HashMap::new();
        let (kmer_tmp, count_tmp): (Vec<String>, Vec<usize>) = count_kmers(&seq[i], *k);
        for j in 0..kmer_tmp.len() {
            kmer_count.insert(kmer_tmp[j].clone(), count_tmp[j]);
        }
        let seq_tmp: Sequence = Sequence {
            header: headers[i].clone(),
            seq: seq[i].clone(),
            kmers: kmer_count
        };

        seqs_kmers.push(seq_tmp);

        // println!("Sequence: {}", headers[i]);
        // println!("K-mer\tCount");
        // for j in 0..kmer_tmp.len() {
        //     println!("{}\t{}", kmer_tmp[j], count_tmp[j]);
        // }

    }
}