use std::collections::HashSet;

fn permutations_with_repeats(k: usize) -> Vec<_> {
    let hashset = ['A', 'T', 'C', 'G'];
    if k == 0 {
        return vec![Vec::new()];
    }

    let mut result = vec![];

    for element in hashset {
        for i in permutations_with_repeats(k - 1) {
            result.insert(0, i)
        }
    }
}
fn main() {
    let hashset: Vec<char> = ['A', 'T', 'C', 'G'].iter().cloned().collect();
    let n = 3;
    permutations_with_repeats(3)
}
