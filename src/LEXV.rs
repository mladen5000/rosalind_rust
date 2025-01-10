use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Add;
use std::result;
use std::vec;

/// This task involves 1. generating all possible permutations of a string (length n) from length 1 - n
/// 2. Sorting these based on the order the were given, not their regular alphabetical order
/// D N A (D > N > A)
///D DD DDD DDN DDA DN DND DNN DNA DA DAD DAN DAA N ND...

#[derive(Debug, Clone)]
struct Alphabet(Vec<Letter>);
impl PartialOrd for Alphabet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Alphabet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialEq for Alphabet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Alphabet {}
impl Add for Alphabet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Alphabet([self.0, rhs.0].concat())
    }
}
impl Iterator for Alphabet {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}
impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_string = self.0.iter().map(|l| l.0).collect::<String>();
        write!(f, "{}", result_string)
    }
}
impl Alphabet {
    pub fn new(letters: Vec<Letter>) -> Self {
        Alphabet(letters)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new_from_str(source_alphabet: &str) -> Self {
        let mut alphabet_vec = vec![];
        for (pos, c) in source_alphabet.char_indices() {
            alphabet_vec.push(Letter(c, pos as u8));
        }
        Alphabet::new(alphabet_vec)
    }

    pub fn build_map(&self) -> HashMap<char, Letter> {
        let mut map = HashMap::new();
        // for letter_struct in self.0 {
        //     map.insert(letter_struct.0, letter_struct);
        // }
        let _: Vec<_> = self
            .0
            .iter()
            .map(|letter| map.insert(letter.0.clone(), *letter))
            .collect();
        map
    }
}
impl From<Alphabet> for HashSet<Letter> {
    fn from(value: Alphabet) -> Self {
        let mut hs = HashSet::new();
        // for i in value.0 {
        //     hs.insert(i)
        // }
        let _: Vec<_> = value
            .0
            .into_iter()
            .map(|letter| hs.insert(letter))
            .collect();
        hs
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Letter(char, u8);

impl Ord for Letter {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
fn permute_them_all_with_repeats(alphabit: Alphabet, max_depth: usize) -> Vec<Alphabet> {
    let vec_letters = &alphabit.0;

    // Base case: If max_depth is 0, return a single empty permutation
    if max_depth == 0 {
        return vec![Alphabet(vec![])];
    }

    let mut perms = Vec::new();

    // Loop through each letter in the input
    for i in 0..vec_letters.len() {
        let fixed = vec_letters[i]; // Fix the current letter

        // Recursively generate permutations with the fixed letter
        let rest_permutations = permute_them_all_with_repeats(alphabit.clone(), max_depth - 1);

        // Combine the fixed letter with each recursive permutation
        for perm in rest_permutations {
            let mut new_perm = vec![fixed];
            new_perm.extend(perm.0); // Append the rest of the permutation
            perms.push(Alphabet(new_perm));
        }
    }

    perms
}

pub fn main() {
    let input = "AZQ";
    let alphabet = Alphabet::new_from_str(input);

    // Generate permutations of all lengths (1 to 3 for DNA)
    let mut all_perms = Vec::new();
    for depth in 1..=alphabet.len() {
        let perms = permute_them_all_with_repeats(alphabet.clone(), depth);
        all_perms.extend(perms);
    }

    // Sort the results based on custom order
    all_perms.sort();

    // Print the sorted results
    for perm in all_perms {
        println!("{}", perm);
    }
}
// println!("{lex_set:?}");
// lex_set.get(&Letter('A'))

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn check_dna_ordering() {
        let input_string = "AZQ";
        let letters = Alphabet::new_from_str(&input_string);
        assert!(letters.0[0] < letters.0[1]);
    }
}
