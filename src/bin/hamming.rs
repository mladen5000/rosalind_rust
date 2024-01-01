fn main() {
    let filepath = "/Users/mladenrasic/Downloads/rosalind_hamm.txt";
    let sequences = std::fs::read_to_string(filepath).expect("Cant find file");

    let mut sequences = sequences.split_whitespace();
    let seq1 = sequences.next().unwrap();
    let seq2 = sequences.next().unwrap();
    println!("{}", seq1);
    println!("{}", seq2);

    // println!("Hello World");
    // let seq1 = "GAGCCTACTAACGGGAT";
    // let seq2 = "CATCGTAATGACGGCCT";
    let result = hamming_distance(seq1, seq2);
    println!("Hamming distance: {}", result);
}

fn hamming_distance(seq1: &str, seq2: &str) -> usize {
    // Check for equal lengths
    assert_eq!(
        seq1.len(),
        seq2.len(),
        "lengths must be equal for hamming distance. The respective lengths are {} , {}",
        seq1.len(),
        seq2.len()
    );

    // Calculate hamming
    seq1.chars()
        .zip(seq2.chars())
        .map(|(i, j)| if i == j { 0 } else { 1 })
        .sum()
}
