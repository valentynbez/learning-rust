use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use flate2::read::GzDecoder;
use std::path::Path;


fn open_filestream (filename: &str) -> BufReader<Box<dyn Read>> {
    let path: &Path = Path::new(filename);
    let file: File = match File::open(&path) {
        Err(why) => panic!("Couldn't open file {}: {}", filename, why),
        Ok(file) => file,
    };
    let filestream: Box<dyn Read> = match filename.ends_with(".gz") {
        true => Box::new(GzDecoder::new(file)),
        false => Box::new(file),
    };
    let reader: BufReader<Box<dyn Read>> = BufReader::new(filestream);
    return reader
}


pub fn parse_fasta (filename: &str) -> (Vec<String>, Vec<String>) {
    let reader: BufReader<Box<dyn Read>> = open_filestream(filename);
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