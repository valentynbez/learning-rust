// write a FASTA parser
// test file

use std::env;
use std::collections::HashMap;
use kmer_counter::parser::parse_fasta;

struct Sequence {
    header: String,
    seq: String,
    kmers: HashMap<String, usize>
}

fn count_kmers (seq: &str, k: usize) -> (Vec<String>, Vec<usize>) {
    let mut kmer: Vec<String> = Vec::new();
    let mut count: Vec<usize> = Vec::new();
    for i in 0..seq.len() - k + 1 {
        let mut kmer_tmp: String = String::new();
        for j in i..i + k {

            kmer_tmp.push(seq.chars().nth(j).unwrap());
        }

        if kmer_tmp.contains('N') {
            continue;
        }
        else if kmer.contains(&kmer_tmp) {
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

        println!("Sequence: {}", headers[i]);
        println!("K-mer\tCount");
        for j in 0..kmer_tmp.len() {
            println!("{}\t{}", kmer_tmp[j], count_tmp[j]);
        }
    }
}