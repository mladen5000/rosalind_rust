mod nucleotide_counter;

use itertools::Itertools;
use std::collections::HashMap; // itertools = "0.10"
use std::fs::{self, File};
use std::hash::Hash;
// use std::fs::File;
use std::io::{BufRead, BufReader};

fn reverse_complement() {
    // Problem 3 Complementing a Strand of DNA
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_revc-3.txt";
    let my_seq: String = fs::read_to_string(my_path)
        .expect("Can't find the file dumbo")
        .trim()
        .to_string();

    // more advanced
    let complement: String = my_seq
        .chars()
        .map(|c: char| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c,
        })
        .collect();

    {

        // This doesn't work bc it is iterative
        // let complement = my_seq
        //     .replace("T", "A")
        //     .replace("A", "T")
        //     ...etc

        // However, this works because it has an intermediate
        // let complement = my_seq
        //     .replace("T", "1")
        //     .replace("1", "A")
        //     ...etc
    }

    let reverse_complement: String = complement.chars().rev().collect();
    // println!("{my_seq}");
    println!("");
    println!("");
    println!("REVERSE COMPLEMENT:");
    println!("{reverse_complement}");
}

fn transcribe_dna() {
    // Problem 2 Transcribing DNA into RNA
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_rna.txt";
    let myseq: String = fs::read_to_string(my_path)
        .expect("Can't find the file dumbo")
        .trim()
        .to_string();
    println!("{}", myseq.replace("T", "U"));
}


fn computing_gc_content() {
    // Problem 4 Computing GC Content
    let my_path: &str = "/Users/mladenrasic/Downloads/rosalind_gc.txt";
    let f: File = File::open(my_path).expect("Couldn't open the file");
    let f: BufReader<File> = BufReader::new(f);

    // Parse each line in fasta
    for line in f.lines() {
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
    // counting_dna_nucleotides(); // #1
    // transcribe_dna(); // #2
    // reverse_complement() // #3
    // computing_gc_content() // #4
    let input = "/Users/mladenrasic/Downloads/rosalind_prot.txt";
    // translate(input);
}
