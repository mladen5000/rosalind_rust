use itertools::Itertools;

/// Given: Two DNA strings s and t
/// Return: All locations of t as a substring of s.
/// seq1 = "GATATATGCATATACTT";
/// seq2 = "ATAT"; // 2 4 10
fn main() {
    // let seq = "GATATATGCATATACTT".to_string();
    // let substr = "ATAT".to_string();

    let filepath = "/Users/mladenrasic/Downloads/rosalind_subs.txt";
    let (seq, substr) = seqs_from_file(filepath);
    let indices = substring_position(seq, substr);

    // Output in rosalind format
    for i in indices.iter() {
        print!("{i} ");
    }
    println!("");
}

fn seqs_from_file(filepath: &str) -> (String, String) {
    let file_string = std::fs::read_to_string(filepath).expect("Can't find filepath");
    let mut lines = file_string.split_whitespace();
    let sequence = lines.next().unwrap().to_string();
    let subsequence = lines.next().unwrap().to_string();
    (sequence, subsequence)
}

fn substring_position(seq: String, substr: String) -> Vec<usize> {
    // Convert to Vec<char>
    let seq: Vec<char> = seq.chars().collect();
    let substr: Vec<char> = substr.chars().collect();
    let n = substr.len();

    // Create windows for sequence (enumerated) and subsequence
    let win1: Vec<(usize, &[char])> = seq.windows(n).enumerate().collect();
    let win2: Vec<&[char]> = substr.windows(n).collect(); // [a,t,a,t]

    // Iterate through each window and identify windows that match the subsequence
    let mut indices: Vec<usize> = vec![];
    for (i, window) in win1.iter() {
        if window == &win2[0] {
            indices.push(*i + 1);
        }
    }
    indices
}

fn substring_position_streamlined(seq: String, substr: String) -> Vec<usize> {
    let n = substr.len();
    let mut indices: Vec<usize> = vec![];

    for (i, window) in seq.as_bytes().windows(n).enumerate() {
        if window == substr.as_bytes() {
            indices.push(i + 1)
        }
    }
    indices
}
