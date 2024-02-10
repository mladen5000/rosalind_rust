use bio::bio_types::sequence::SequenceRead;
use bio::io::fastq;
use needletail::parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    id: Vec<u8>,
    seq: Vec<u8>,
}
fn main() {
    let mut full_vec: Vec<Node> = vec![];

    // Parse the FASTQ file
    let reader = BufReader::new(File::open("rosalind_grph.txt").expect("Failed to open file"));
    let parser = fastq::Reader::new(reader);

    // Iterate over the records
    for record in parser.records() {
        let record = record.expect("Failed to parse record");

        // Create a new node
        let end = record.id().len() - 3;
        let node = Node {
            id: record.id().to_string()[..end],
            seq: record.seq().to_owned[..end],
        };

        // Add the node to the vector
        full_vec.push(node);
    }

    // Process each record
    println!("{:?}", full_vec);
}
