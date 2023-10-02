use std::str;
use std::{collections::HashMap, hash::Hash};

use needletail::{errors::ParseError, parse_fastx_file, Sequence};

const KMER_SIZE: u8 = 4;

struct KmerCounter {
    kmer_counts: HashMap<String, usize>,
    n_bases: usize,
    n_valid_kmers: usize,
}

impl KmerCounter {
    fn new() -> Self {
        KmerCounter {
            kmer_counts: HashMap::new(),
            n_bases: 0,
            n_valid_kmers: 0,
        }
    }
    fn sort(&self) -> Vec<(&String, &usize)> {
        let mut kmer_vec: Vec<_> = self.kmer_counts.iter().collect();
        kmer_vec.sort_by(|a, b| b.1.cmp(&a.1));
        kmer_vec
    }
    fn print(&self) {
        // println!("There are {self.n_bases} bases in your file.");
        // println!("There are {self.n_valid_kmers} valid k-mers in your file.");
        println!("{:?}", self.sort());
    }
    fn add_kmer(&mut self, kmer: String) {
        *self.kmer_counts.entry(kmer).or_insert(0) += 1;
    }
    fn add_bases(&mut self, n: usize) {
        self.n_bases += n;
    }
    fn get_kmer_counts(&self) -> &HashMap<String, usize> {
        &self.kmer_counts
    }
    fn get_n_bases(&self) -> usize {
        self.n_bases
    }
    fn get_n_valid_kmers(&self) -> usize {
        self.n_valid_kmers
    }
    fn get_top_kmer(&self) -> (&String, &usize) {
        self.kmer_counts
            .iter()
            .max_by(|k, v| k.1.cmp(&v.1))
            .unwrap()
    }
    fn get_top_kmer_count(&self) -> usize {
        *self
            .kmer_counts
            .iter()
            .max_by(|k, v| k.1.cmp(&v.1))
            .unwrap()
            .1
    }
    fn build_from_file<'a>(&mut self, file_path: &'a str) {
        let mut n_valid_kmers = 0;
        // let mut kmer_counter = HashMap::new();
        let mut reader = parse_fastx_file(&file_path).expect("File path should be a valid path.");
        while let Some(record) = reader.next() {
            let seqrec = record.expect("Should be a valid record");
            let norm_seq = seqrec.normalize(false);
            let rc = norm_seq.reverse_complement();
            for (_, kmer, _) in norm_seq.canonical_kmers(KMER_SIZE, &rc) {
                n_valid_kmers += 1;
                // *kmer_counter
                //     .entry(String::from_utf8(kmer.to_vec()).unwrap())
                //     .or_insert(0) += 1;
            }
        }
    }
}

pub fn build_kmer_counter<'a>(file_path: &'a str) -> HashMap<String, usize> {
    // Initialize variables
    let mut n_valid_kmers = 0;
    let mut kmer_counter = HashMap::new();

    // Create a reader for the fastq file
    let mut reader = parse_fastx_file(&file_path).expect("File path should be a valid path.");

    // Loop through the records
    while let Some(record) = reader.next() {
        let seqrec = record.expect("Should be a valid record");
        let norm_seq = seqrec.normalize(false);
        let rc = norm_seq.reverse_complement();
        for (_, kmer, _) in norm_seq.canonical_kmers(KMER_SIZE, &rc) {
            n_valid_kmers += 1;
            *kmer_counter
                .entry(String::from_utf8(kmer.to_vec()).unwrap())
                .or_insert(0) += 1;
        }
    }
    let top_kmer = kmer_counter.iter().max_by(|k, v| k.1.cmp(&v.1)).unwrap();

    // println!("There are {n_bases} bases in your file.");
    println!("There are {n_valid_kmers} valid k-mers in your file.");
    println!("{:?}", top_kmer);
    kmer_counter
}

// pub fn sort_kmer_counter(kmer_map: HashMap<_, _>) {
//     let mut kmer_vec: Vec<_> = kmer_map.into_iter().collect();
//     kmer_vec.sort_by(|a, b| b.1.cmp(&a.1));
//     println!("{:?}", kmer_vec);
// }

fn main() {
    let fpath = "/Users/mladenrasic/Downloads/sample.fastq";
    let kmer_counts = build_kmer_counter(fpath);
    println!("The hashmap is {:?}.", kmer_counts);
}

pub fn read_parser(forward_path: &str, reverse_path: &str) -> Result<(), ParseError> {
    let mut forward = parse_fastx_file(&forward_path)?;
    let mut reverse = parse_fastx_file(&reverse_path)?;

    // Loop through the records
    while let Some(fseq) = forward.next() {
        let fseq = fseq.clone()?;
        while let Some(rseq) = reverse.next() {
            let rseq = rseq.clone()?;
            let joined_sequence = std::str::from_utf8(fseq.sequence()).unwrap().to_owned()
                + "\nNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN\n"
                + std::str::from_utf8(rseq.sequence()).unwrap()
                + "\n";

            println!("{}", joined_sequence)
        }
    }
    Ok(())
}
