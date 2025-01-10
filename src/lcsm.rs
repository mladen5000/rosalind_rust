/// Generate a generalized suffix array from multiple strings
pub fn run() {
    let test_seqs = vec![
        String::from("tea"),
        String::from("tan"),
        String::from("ted"),
        String::from("tad"),
        String::from("tom"),
        String::from("bread"),
        String::from("related"),
    ];
    build_generalized_suffix_array(&test_seqs[..]);

    // let mut test_seq = String::from("banana");
    // build_suffix_array(&mut test_seq);
}

fn build_suffix_array(seq: &mut String) -> (Vec<usize>, Vec<&str>) {
    // Comment for neatness
    seq.push('$');

    // Generate initial unsorted suffix array
    let mut suffix_array: Vec<(_, _)> = (0..seq.len())
        .enumerate()
        .map(|(num, i)| (num, &seq[i..]))
        .collect();

    // Sort both suffixes and their original starting position, then split into 2 vecs
    suffix_array.sort_by(|a, b| a.1.cmp(&b.1));
    let (suffix_array, text): (Vec<_>, Vec<_>) = suffix_array.into_iter().unzip();

    // TODO: debug stmt
    println!(
        "
        sorted suffixes: {text:#?}
        suffix_array: {suffix_array:?}
        "
    );

    (suffix_array, text)
}

fn build_generalized_suffix_array(seqs: &[String]) {
    let (mut sequence, _positions) = concatenate_seqs(seqs);

    let (sa, words) = build_suffix_array(&mut sequence);

    // Generate LCP array
    let mut lcp_vec = vec![];
    for i in 0..(words.len() - 1) {
        let lcp = longest_common_prefix(words[i], words[i + 1]);
        lcp_vec.push(lcp);
    }
    println!("lcp_vec: {:?}", lcp_vec);

    assert_eq!(sa.len(), lcp_vec.len() + 1)
}

/// Generate single string (with unique delimeters) from slice of strings
fn concatenate_seqs(seqs: &[String]) -> (String, Vec<(usize, usize)>) {
    let mut big_seq = String::default();

    // Append an enumerated delimeter to each sequence and push to larger String
    let mut seq_positions = vec![];
    for (i, s) in seqs.into_iter().enumerate() {
        let mut s = s.clone();
        s.push('$');
        s.push_str(&i.to_string());
        let prev_len = big_seq.len();
        big_seq.push_str(&s);
        let new_len = big_seq.len();
        seq_positions.push((prev_len, new_len));
    }
    // TODO: debug stmt
    println!("Full Sequence: {}", big_seq);
    println!("Sequence Positions: {:?}", seq_positions);

    (big_seq, seq_positions)
}

fn longest_common_prefix(suffix1: &str, suffix2: &str) -> usize {
    // println!("{suffix1} {suffix2}");
    let lcp = suffix1
        .chars()
        .zip(suffix2.chars())
        .map_while(|(c1, c2)| {
            if c1 == c2 {
                // println!("{c1} {c2}");
                Some(c1)
            } else {
                None
            }
        })
        .count();
    // println!("{lcp}");
    lcp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_basic_sa() {
        let mut input = "blah".to_string();
        build_suffix_array(&mut input);
    }
}
