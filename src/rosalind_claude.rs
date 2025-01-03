// Problem: DNA Motif Counter
// ==========================
// Given a DNA sequence and a motif (both as strings), count how many times
// the motif appears in the sequence, including overlapping occurrences.
// The sequence will only contain valid DNA nucleotides (A, C, G, T).
// The motif must be an exact match (case-sensitive).
//
// Example:
// Sequence: "ACGTACGTACGT"
// Motif: "ACGT"
// Result: 3
//
// Write a function that takes two string slices and returns the count.

pub fn count_motif(sequence: &str, motif: &str) -> usize {
    let mut count = 0;
    if sequence.len() == 0 || motif.len() == 0 {
        return 0;
    }

    for substr in sequence.as_bytes().windows(motif.len()) {
        if substr.to_ascii_uppercase() == motif.as_bytes() {
            count += 1;
            println!("match");
        } else {
            println!("no match");
        }
    }
    println!("{count}");
    count
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(count_motif("ACGTACGTACGT", "ACGT"), 3);
        assert_eq!(count_motif("AAAA", "AA"), 3); // Note: overlapping matches!
        assert_eq!(count_motif("GATTACA", "TT"), 1);
        assert_eq!(count_motif("GATTACA", "TTT"), 0);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(count_motif("", "A"), 0);
        assert_eq!(count_motif("ACGT", ""), 0);
        assert_eq!(count_motif("A", "AA"), 0);
    }

    #[test]
    fn test_complex_cases() {
        assert_eq!(count_motif("ATATATA", "ATA"), 3);
        assert_eq!(count_motif("GCGCGCGC", "GCGC"), 3);
        assert_eq!(count_motif("TCTCTCTC", "TCT"), 3);
    }
}
