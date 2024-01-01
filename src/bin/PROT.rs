use std::collections::HashMap;

fn main() {
    // Codon map
    let filepath = "/Users/mladenrasic/Projects/rosalind_rust/prot_rna_table.txt";
    let codon_map = build_codon_map(filepath);

    // Input file
    let sequencepath = "/Users/mladenrasic/Downloads/rosalind_prot-1.txt";
    let mrna = get_sequence_from_file(sequencepath);

    // Translation of mrna
    let aa_sequence = get_aa_sequence(mrna, &codon_map);
    println!("{:?}", aa_sequence);
}

fn build_codon_map<'a>(filepath: &str) -> HashMap<String, String> {
    let file = std::fs::read_to_string(filepath).expect("Can't find file");
    let mut codon_map = HashMap::new();
    let file_lines = file.lines(); // Create an iterator over the lines of the file
    for line in file_lines {
        let mut splits = line.split_whitespace();
        let first = splits.next().unwrap().to_string();
        let second = splits.next().unwrap().to_string();
        codon_map.insert(first, second);
    }
    codon_map
}

fn get_sequence_from_file(filepath: &str) -> String {
    let mrna = std::fs::read_to_string(filepath).expect("Can't find sequence file");
    mrna
}

fn get_aa_sequence(sequence: String, codon_map: &HashMap<String, String>) -> String {
    sequence
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|chunk| codon_map.get(&chunk.iter().collect::<String>()))
        .take_while(|&amino_acid| amino_acid != "Stop")
        .map(|amino_acid| amino_acid.to_string())
        .collect()
}
