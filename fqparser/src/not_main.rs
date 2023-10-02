use crate::KMER_SIZE;
use bio::io::fastq::Error;
use needletail::{errors::ParseError, parse_fastx_file, Sequence};
use std::collections::HashMap;

pub fn run_needletail(filename: &str) -> Result<HashMap<String, usize>, Error> {
    // fastest
    let mut kmer_dict: HashMap<String, usize> = HashMap::new();
    let mut nbases = 0;
    let mut nkmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("valid path");

    // Loop through the records
    while let Some(record) = reader.next() {
        // Get the record
        let seqrec = record.expect("Should be a valid record");
        // Make all caps and trim whitespace
        let norm_seq = seqrec.normalize(false);
        // Get the reverse complement
        let rc = norm_seq.reverse_complement();

        // Loop through and store kmers
        // for (_, kmer, _) in norm_seq.canonical_kmers(KMER_SIZE.try_into().unwrap_or(4), &rc) {
        for kmer in norm_seq.kmers(KMER_SIZE.try_into().unwrap_or(4)) {
            // Convert kmer from &[u8] to String
            let kmer_str = std::str::from_utf8(kmer).unwrap().to_owned();
            // Insert k-mer to dictionary and increase count by 1
            *kmer_dict.entry(kmer_str).or_insert(0) += 1;
            // Increase k-mer count
            nkmers += 1;
        }
    }

    // Number of unique k-mers
    let nkmers_uniq = kmer_dict.len();

    // Print the results
    println!("Number of kmers: {:?}", nkmers);
    println!("Number of unique kmers: {:?}", nkmers_uniq);

    // Return result hashmap
    Ok(kmer_dict)
}

pub fn read_fq() -> Result<(), Error> {
    let mut x: [i32; 5] = [1, 2, 3, 4, 5];
    let mut v = vec![1, 2, 3, 4, 5];

    let mut kmer_dict: HashMap<String, usize> = HashMap::new();
    let mut nbases = 0;
    let mut nkmers = 0;
    let filename = "/Users/mladenrasic/Downloads/sample.fastq";

    let mut fastx_file = parse_fastx_file(&filename)
        .expect("Can't open {filename}, are you sure thats the right path?");

    while let Some(record) = fastx_file.next() {
        let record = record.expect("Invalid record");

        // Read a sequence fastq file
    }
    Ok(())
}
