use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Add;

/// This task involves 1. generating all possible permutations of a string (length n) from length 1 - n
/// 2. Sorting these based on the order the were given, not their regular alphabetical order
/// D N A (D > N > A)
///D DD DDD DDN DDA DN DND DNN DNA DA DAD DAN DAA N ND...

#[derive(Debug, Clone)]
struct Alphabet(Vec<Letter>);

impl Add for Alphabet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Alphabet([self.0, rhs.0].concat())
    }
}

impl Iterator for Alphabet {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = 0;
        if current < self.0.len() {
            let result = self.0[current].clone(); // Use clone if T: Clone
            current += 1;
            Some(result)
        } else {
            None // End of iteration
        }
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.0);
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
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn permute_them_all(alphabit: Alphabet) -> Vec<Alphabet> {
    if alphabit.0.len() == 0 {
        return vec![Alphabet(vec![])];
    }
    let mut perms = vec![];

    for i in 0..alphabit.0.len() {
        let fixed = alphabit.0[i];
        let rest = Alphabet([alphabit.0[..i].to_vec(), alphabit.0[1 + i..].to_vec()].concat());

        let rest_permutations = permute_them_all(rest);

        for perm in rest_permutations {
            let mut new_perm = vec![fixed];
            new_perm.extend(perm.0); // Append rest of the permutation
            perms.push(Alphabet(new_perm));
        }
    }
    perms
}

pub(crate) fn build_it_up(sequence: &str) -> Alphabet {
    Alphabet::new_from_str(&sequence)
}

pub fn main() {
    // let sequence = "DNA";
    let sequence = "EZRW"; //BDOMIS";
    let alpha = build_it_up(sequence);
    let mut all_perms = permute_them_all(alpha);
    let mut results = vec![];

    all_perms.sort_by(|a, b| a.clone().cmp(b.clone()));
    for alphabit in all_perms {
        let result: String = alphabit.0.iter().map(|i| i.0).collect();
        results.push(result);
    }
    for r in results {
        println!("{r:?}");
    }
}
// println!("{lex_set:?}");
// lex_set.get(&Letter('A'))

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn check_dna_ordering() {
        let input_string = "DNA";
        let letters = Alphabet::new_from_str(&input_string);
        assert!(letters.0[0] < letters.0[1]);
    }
}
