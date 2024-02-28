use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use std::iter::Iterator; // Add the Iterator trait import

fn main() {
    let input_string = String::from("AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG");
    let codon_aa_map = build_codon_aa_map();

    for i in 0..=2 {
        // Forward strand
        let frame = &transcribe(&input_string)[i..];
        let protein_string = convert_codons(frame, &codon_aa_map);
        let subseq = start_stop(&protein_string);

        // Reverse complement strand
        let frame = &reverse_complement(&input_string)[i..];
        let protein_string = convert_codons(frame, &codon_aa_map);
        let subseq = start_stop(&protein_string);
        // println!("{subseq}");
    }
}

fn build_codon_aa_map<'a>() -> HashMap<&'a str, &'a str> {
    let codon_aa_map = HashMap::from([
        ("UUU", "F"),
        ("UUC", "F"),
        ("UUA", "L"),
        ("UUG", "L"),
        ("CUU", "L"),
        ("CUC", "L"),
        ("CUA", "L"),
        ("CUG", "L"),
        ("AUU", "I"),
        ("AUC", "I"),
        ("AUA", "I"),
        ("AUG", "M"),
        ("GUU", "V"),
        ("GUC", "V"),
        ("GUA", "V"),
        ("GUG", "V"),
        ("UCU", "S"),
        ("UCC", "S"),
        ("UCA", "S"),
        ("UCG", "S"),
        ("CCU", "P"),
        ("CCC", "P"),
        ("CCA", "P"),
        ("CCG", "P"),
        ("ACU", "T"),
        ("ACC", "T"),
        ("ACA", "T"),
        ("ACG", "T"),
        ("GCU", "A"),
        ("GCC", "A"),
        ("GCA", "A"),
        ("GCG", "A"),
        ("UAU", "Y"),
        ("UAC", "Y"),
        ("UAA", "Stop"),
        ("UAG", "Stop"),
        ("CAU", "H"),
        ("CAC", "H"),
        ("CAA", "Q"),
        ("CAG", "Q"),
        ("AAU", "N"),
        ("AAC", "N"),
        ("AAA", "K"),
        ("AAG", "K"),
        ("GAU", "D"),
        ("GAC", "D"),
        ("GAA", "E"),
        ("GAG", "E"),
        ("UGU", "C"),
        ("UGC", "C"),
        ("UGA", "Stop"),
        ("UGG", "W"),
        ("CGU", "R"),
        ("CGC", "R"),
        ("CGA", "R"),
        ("CGG", "R"),
        ("AGU", "S"),
        ("AGC", "S"),
        ("AGA", "R"),
        ("AGG", "R"),
        ("GGU", "G"),
        ("GGC", "G"),
        ("GGA", "G"),
        ("GGG", "G"),
    ]);
    codon_aa_map
}

fn convert_codons(frame: &str, codon_aa_map: &HashMap<&str, &str>) -> String {
    let protein_string: String = frame
        .chars()
        .chunks(3)
        .into_iter()
        .filter_map(|c| {
            let chunk: String = c.collect();
            Some(
                codon_aa_map
                    .get(&chunk.to_string().as_str())
                    .cloned()
                    .unwrap_or("."),
            )
        })
        .collect();
    protein_string
}

fn reverse_complement(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' => 'U',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        })
        .collect::<String>()
}

fn transcribe(seq: &str) -> String {
    seq.chars()
        .map(|c| match c {
            'T' => 'U',
            x => x,
            _ => unreachable!(),
        })
        .collect::<String>()
}

fn start_stop(seq: &str) -> String {
    let seq = match seq.find("M") {
        Some(pos) => &seq[pos..],
        None => seq,
    };
    let seq = match seq.find("Stop") {
        Some(pos) => &seq[..pos],
        None => seq,
    };
    println!("The sequence with the new start and stop is: {seq}");
    seq.to_string()
}
