// Problem 1: Counting DNA Nucleotides
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

/// Problem 1 Counting DNA Nucleotides
/// Returns the number of nucleotides in a sequence
pub fn counting_dna_nucleotides() -> HashMap<char, i32> {
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

    map
}

fn main() {
    let dna_count = counting_dna_nucleotides();

    // Print out the values
    for (k, v) in dna_count.iter().sorted_by_key(|x| x.0) {
        println!("{k} : {v}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_dna_nucleotides() {
        let expected_res = HashMap::from([('A', 225), ('C', 224), ('G', 216), ('T', 206)]);
        assert_eq!(expected_res, counting_dna_nucleotides());
    }
}
