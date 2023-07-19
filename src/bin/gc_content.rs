// Problem 4 - Computing the GC content of a DNA string
use std::io::{BufRead, BufReader,Error};
use std::fs::File;

//TODO - Implement function for each fasta sequence as opposed to per file
//TODO - Or simply retain the max gc content and sequence
pub fn computing_gc_content(filepath: &str) -> Result<f32, Error> {
    let f = File::open(filepath)?;
    let buffered = BufReader::new(f);
    "/home/mll"

    // Initialize count variables
    let mut gc_count = 0.0;
    let mut total_count = 0.0;

    // Parse each line in fasta
    for line in buffered.lines() {
        let line = line?;

        // Skip header line
        if line.starts_with(">") {
            continue;
        }

        // Count GC content
        gc_count = line.bytes().filter(|c| (*c == b'C' || *c == b'G')).count() as f32;
        total_count = line.bytes().count() as f32;
    }

    // Get GC fraction
    let gc_content = gc_count / total_count;
    Ok(gc_content)

    //TODO - I think I need to return the sequence with largest gc content
}

fn main() {
    let gc_file = "/Users/mladenrasic/Downloads/rosalind_gc.txt";
    let result = computing_gc_content(gc_file).expect("Error computing GC content");
    println!("{result}")
}
