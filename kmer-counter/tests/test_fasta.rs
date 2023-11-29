use std::io::{BufRead, BufReader};

extern crate kmer_counter;
use kmer_counter::parser::collect_headers_sequences;

const FASTA: &[&[u8]; 8] = &[
    b">test_genome1 desc",
    b"AGTTTTTCAGAGAGCTAG",
    b">test_genome2",
    b"CCCCCGAGAGAGTCAGNN",
    b">test_genome1",
    b"AGTTTTTCAGAGAGCTAG",
    b">test_genome3",
    b"CCCNNNNATTTTTGNTTT"
];

#[test]
fn test_collect_headers_sequences () {
    let reader: BufReader<&[u8]> = BufReader::new(FASTA.join(&b"\n"[..]));
    let (headers, seq) = collect_headers_sequences(reader);
    assert_eq!(headers, vec!["test_genome1", "test_genome2", "test_genome3"]);
    assert_eq!(seq, vec!["AGTTTTTCAGAGAGCTAG", "CCCCCGAGAGAGTCAGNN", "CCCNNNNATTTTTGNTTT"]);
}
