use std::collections::HashMap; // itertools = "0.10"
use std::fs::{self};
use std::io::{BufRead};

/*
fn translate(input: &str) {
    let rna = fs::read_to_string(input)
        .expect("Unable to read file")
        .trim();

    // for c in rna
    // let protein = rna.chars().chunks(3).get();
    // map(|x| lookup(x)).collect();
}
*/

/*
fn lookup(codon: &str) -> HashMap<&str, &str> {
    let mut codon_map = HashMap::new();

    //    let blah =
}
*/

fn build_codon_table(file: &str) -> Option<HashMap<String, String>> {
    let mut codon_map = HashMap::new();
    for line in fs::read_to_string(file)
        .expect("codon file not found")
        .lines()
    {
        let mut split = line.split_whitespace();
        let codon = split.next().unwrap().to_string();
        let _skip = split.next()?;
        let amino_acid = split.next()?.to_string();
        codon_map.insert(codon, amino_acid);
    }
    println!("{:?}", codon_map);

    Some(codon_map)
}

fn main() {
    let _input = "/Users/mladenrasic/Downloads/rosalind_prot.txt";
    let codon_file = "/Users/mladenrasic/Downloads/codon-1.txt";
    build_codon_table(&codon_file);
    // translate(input);
}
