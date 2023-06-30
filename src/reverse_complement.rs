/// Problem 3 Complementing a Strand of DNA
use std::fs;

fn reverse_complement(dna: String) -> String {

    let complement: String = dna 
        .chars()
        .map(|c: char| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c,
        })
        .collect();


    let reverse_complement: String = complement.chars().rev().collect();
    // println!("{my_seq}");
    reverse_complement
}

fn main() {
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_revc-3.txt";
    let my_seq: String = fs::read_to_string(my_path)
        .expect("Can't find the file dumbo")
        .trim()
        .to_string();

    let rc_dna= reverse_complement(my_seq);

    println!("REVERSE COMPLEMENT:");
    println!("{rc_dna}");
}