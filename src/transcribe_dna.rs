// Problem 2 - Transcribing DNA into RNA
use std::fs;

/// Takes a DNA sequence and returns the RNA sequence
pub fn transcribe_dna(myseq: String) -> String {
    let rna = myseq.replace("T", "U");
    rna
}

fn main() {
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_rna.txt";
    let myseq: String = fs::read_to_string(my_path)
        .expect("Can't find the file dumbo")
        .trim()
        .to_string();
    let rna = transcribe_dna(myseq);
    println!("{rna}");
}
