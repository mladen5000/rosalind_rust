// Problem 4 - Computing the GC content of a DNA string
use std::io::{BufRead, BufReader};


fn computing_gc_content() {
    // Problem 4 Computing GC Cntent
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_gc.txt";
    let f = File::open(my_path)?;
    let buffered  = BufReader::new(f);

    // Parse each line in fasta
    for line in buffered.lines() {
        let line: String = line.expect("Unable to read line");

        // initialize gc and total

        // Header line
        if line.starts_with(">") {
            continue;
        }

        //Calculate number of GC nucleotides
        let gc_count: f32 = line.chars().filter(|c| (*c == 'C' || *c == 'G')).count() as f32;

        // Calculate NT NUM
        let total: usize = line.chars().count();

        let gc_count: f32 = gc_count as f32;
        let total: f32 = total as f32;
        let gc: f32 = gc_count / total;

        println!("GC Content: {}", gc);
        //TODO - Return largest fasta
    }
}

fn main() {
    computing_gc_content()
}