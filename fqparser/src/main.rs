mod lib;
mod not_main;

use bio::io::fastq;
use bio::io::fastq::Error;
use dashmap::DashMap;
use needletail::kmer;
use needletail::{errors::ParseError, parse_fastx_file, Sequence};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;

const KMER_SIZE: usize = 8;

fn dict_last(fpath: &str) -> Result<(), Error> {
    // This approach is the fastest of the two
    // File IO
    let fpath = "/Users/mladenrasic/Downloads/sample.fastq";
    let _f = File::open(fpath).expect("Cannot open file at {fpath}");
    let reader = fastq::Reader::from_file(fpath).expect("Cannot open file at {fpath}");
    // let reader = fastq::Reader::new(f).expect("Cannot open file at {fpath}");

    // Collect records
    let records: Vec<_> = reader.records().collect();

    // Calculate kmers
    let kmers: Vec<String> = records
        .iter()
        // For each record
        .flat_map(|result| {
            // record = borrow result
            let record = result.as_ref().expect("No record found");

            record
                // Get the sequence
                .seq()
                // Generate a sliding window
                .windows(KMER_SIZE)
                // K-mer u8 to String
                .map(|kmer| {
                    std::str::from_utf8(kmer)
                        .expect("[u8] should be converted to a string slice")
                        .to_string()
                })
                // Collect
                .collect::<Vec<String>>() // Collect the kmers
        })
        .collect();

    // Step 2: Build the hashmap
    let mut kmer_counts = HashMap::new();
    for kmer in kmers {
        *kmer_counts.entry(kmer).or_insert(0) += 1;
    }

    println!("Number of kmers: {:?}", kmer_counts.len());
    Ok(())
}

// Naive approach -
fn standard() -> Result<(), Error> {
    // create FASTQ reader
    let fpath = "/Users/mladenrasic/Downloads/sample.fastq";
    let reader = fastq::Reader::from_file(fpath).expect("Cannot open file at {fpath}");

    let kmer_counts: DashMap<String, usize> = DashMap::new();

    // Convert the records into a Vec so that we can use par_iter
    let records: Vec<_> = reader.records().collect();

    // Use par_iter to process the records in parallel
    let counts: Vec<DashMap<String, usize>> = records
        .par_iter()
        .map(|result| {
            let record = result.as_ref().expect("No record found");

            let local_counts = DashMap::new();

            record.seq().windows(KMER_SIZE).for_each(|kmer| {
                let kmer = std::str::from_utf8(kmer).unwrap().to_string();
                let mut count = local_counts.entry(kmer).or_insert(0);
                *count += 1;
            });

            local_counts
        })
        .collect();

    // Merge the local counts into the global counts
    for local_counts in counts {
        for (kmer, count) in local_counts {
            *kmer_counts.entry(kmer).or_insert(0) += count;
        }
    }

    println!("Number of kmers: {:?}", kmer_counts.len());

    Ok(())
}

fn run_needletail(filename: &str) -> Result<HashMap<String, usize>, Error> {
    // fastest
    let mut kmer_dict = HashMap::new();
    let mut nbases = 0;
    let mut nkmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("valid path");

    // Loop through the records
    while let Some(record) = reader.next() {
        // Get the record
        let seqrec = record.expect("Should be a valid record");
        // Count bases
        nbases += seqrec.num_bases();
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
    println!("Number of bases: {:?}", nbases);
    println!("Number of kmers: {:?}", nkmers);
    println!("Number of unique kmers: {:?}", nkmers_uniq);

    // Return result hashmap
    Ok(kmer_dict)
}

fn main() -> Result<(), Error> {
    // Run the k-mer counter
    let filename = "/Users/mladenrasic/Downloads/sample.fastq";

    // // let kmer_freq = run_needletail(filename)?;
    // let kmer_freq = not_main::run_needletail(filename)?;

    // // Print if reasonably sized k
    // if KMER_SIZE <= 8 {
    //     println!("K-mer frequences:\n {:?}", kmer_freq);
    // }

    lib::testing(filename);

    Ok(())
}
