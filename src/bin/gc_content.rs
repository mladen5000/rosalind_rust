// Problem 4 - Computing the GC content of a DNA string
use bio::seq_analysis::gc;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

//TODO - Implement function for each fasta sequence as opposed to per file
//TODO - Or simply retain the max gc content and sequence
#[derive(Debug, PartialEq, PartialOrd, Default)]
struct FastaRecord {
    id: String,
    sequence: String,
    gc_content: f32,
}

// Generate ordering based off gc_content
impl Eq for FastaRecord {}

impl Ord for FastaRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.gc_content.partial_cmp(&other.gc_content).unwrap()
    }
}

fn gc_content(seq: String) -> f32 {
    let gc = seq
        .to_uppercase()
        .chars()
        .filter(|&c| c == 'C' || c == 'G')
        .count();
    let ratio = gc as f32 / seq.len() as f32 * 100.0;
    ratio
}

fn maximum_gc(buf: BufReader<File>) -> Result<FastaRecord, Error> {
    // Initialize
    let mut sequence = String::new();
    let mut max_record = FastaRecord::default();
    let mut id: String = String::new();

    // Get first id of file
    let mut lines = buf.lines();
    if let Some(first_line) = lines.next() {
        id = first_line?.replace(">", "");
    }

    // Loop
    for line in lines {
        let line = line?;
        // Keep concatenating DNA until we reach ">seqid"
        if !line.starts_with(">") {
            sequence += &line;
        } else {
            // Calculate gc content of this total seq once we hit sequence id
            let record = FastaRecord {
                id: id.clone(),
                sequence: sequence.clone(),
                gc_content: gc_content(sequence.clone()),
            };
            sequence = String::new();
            println!("{:?}", record);

            if record > max_record {
                max_record = record;
            }

            id = line[1..].to_string();

            // Skip the very first line (NaN)
            // if sequence.is_empty() {
            //     continue;
            // }
            // if id.is_empty() {
            //     continue;
            // }
        }
    }
    // println!("{:?}", gc_content(sequence.to_owned()));
    let record = FastaRecord {
        id: id.clone(),
        sequence: sequence.clone(),
        gc_content: gc_content(sequence.clone()),
    };
    if record > max_record {
        println!("{} > {}", record.gc_content, max_record.gc_content);
        max_record = record;
    }
    println!("{} {}", max_record.id, max_record.gc_content);
    Ok(max_record)
}

fn main() -> Result<(), std::io::Error> {
    // Load file into buffer
    let gc_file = "/Users/mladenrasic/Projects/rosalind_rust/rosalind_gc.txt";
    let buf = File::open(gc_file)
        .map(BufReader::new)
        .expect("\n File not found. Check again? \n\n");
    maximum_gc(buf);
    Ok(())
}
