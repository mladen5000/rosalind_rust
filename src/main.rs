
use std::collections::HashMap; // itertools = "0.10"
use std::fs::{self, File};
// use std::fs::File;
use std::io::{BufRead, BufReader};


fn translate(input: &str) {
    let rna = fs::read_to_string(input)
        .expect("Unable to read file")
        .trim();

    // for c in rna
    // let protein = rna.chars().chunks(3).get();
    // map(|x| lookup(x)).collect();
}

fn lookup(codon: &str) -> HashMap<&str, &str> {
    let mut codon_map = HashMap::new();
    codon_map.insert("UUU", "F");
    codon_map.insert("AUC", "I");
    codon_map
    //    let blah =
}

fn codon_table(file: &str) -> HashMap<&str, &str> {
    let mut codon_map = HashMap::new();
    let f: &str = fs::read_to_string(file)
        .expect("file not found error")
        .trim();
    codon_map
}

fn main() {
    // computing_gc_content() // #4
    let input = "/Users/mladenrasic/Downloads/rosalind_prot.txt";
    // translate(input);
}
