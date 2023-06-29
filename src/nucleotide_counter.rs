// Problem 1: Counting DNA Nucleotides
use std::fs;
use std::collections::HashMap;

fn counting_dna_nucleotides() {
    // Problem 1 Counting DNA Nucleotides
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_dna-1.txt";

    // Grab sequence and clean up
    let myseq: String = fs::read_to_string(my_path)
        .expect("Unable to read file")
        .trim()
        .to_string();

    // Put values into the HashMap
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in myseq.chars() {
        let count: &mut i32 = map.entry(c).or_insert(0);
        *count += 1;
    }

    // Print out the values
    for (k, v) in map.iter().sorted_by_key(|x| x.0) {
        print!("{} ", v);
        println!("");
    }
}

fn main() {
    counting_dna_nucleotides();
}
