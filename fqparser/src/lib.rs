use std::collections::HashMap;
use std::str;

use needletail::{parse_fastx_file, Sequence};

pub fn testing<'a>(file_path: &'a str) -> HashMap<String, usize> {
    // Initialize variables
    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    let mut kmer_counter = HashMap::new();

    // Create a reader for the fastq file
    let mut reader = parse_fastx_file(&file_path).expect("File path should be a valid path.");

    // Loop through the records
    while let Some(record) = reader.next() {
        // Obtain the fastq record
        let seqrec = record.expect("invalid record");

        // keep track of the total number of bases
        n_bases += seqrec.num_bases();

        // Normalize to make sure all the bases are consistently capitalized and
        // that we remove the newlines
        let norm_seq = seqrec.normalize(false);

        // we make a reverse complemented copy of the sequence first for
        // `canonical_kmers` to draw the complemented sequences from.
        let rc = norm_seq.reverse_complement();

        // Obtain the canonical kmer (i.1)
        for (_, kmer, _) in norm_seq.canonical_kmers(31, &rc) {
            //kmer loop
            n_valid_kmers += 1;

            // Convert kmer from &[u8] to String and insert into hashmap
            *kmer_counter
                .entry(String::from_utf8(kmer.to_vec()).unwrap())
                .or_insert(0) += 1;
        }
    }
    let top_kmer = kmer_counter.iter().max_by(|k, v| k.1.cmp(&v.1)).unwrap();
    println!("There are {n_bases} bases in your file.");
    println!("There are {n_valid_kmers} valid k-mers in your file.");
    println!("{:?}", top_kmer);
    kmer_counter
}

fn main() {
    let fpath = "/Users/mladenrasic/Downloads/sample.fastq";
    let mykmer_map = testing(fpath);
    println!("The hashmap is {:?}.", mykmer_map);
    println!("{:?}", mykmer_map);
}
