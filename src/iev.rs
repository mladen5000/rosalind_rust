use std::iter::zip;

///
/// Given: Six nonnegative integers, each of which does not exceed 20,000. The integers correspond to the number of couples in a population possessing each genotype pairing for a given factor. In order, the six given integers represent the number of couples having the following genotypes:
///
///
/// AA-AA AA-Aa AA-aa Aa-Aa Aa-aa aa-aa
/// Return: The expected number of offspring displaying the dominant phenotype in the next generation, under the assumption that every couple has exactly two offspring.

pub fn offspring_probability_helper() {
    // Input handling
    let input_data = std::fs::read_to_string("rosalind_iev.txt").expect("Can't read file");
    let input_vec: Vec<i32> = input_data
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
    let input = input_vec;
    println!("{input:?}");

    // Do stuff
    let p = [1.0, 1.0, 1.0, 0.75, 0.5, 0.0];
    let result = offspring_probability(&input, &p);
    println!("{result:?}");
}

fn offspring_probability(start: &[i32], probs: &[f64]) -> f64 {
    let mut total = 0f64;
    for (s, p) in zip(start, probs) {
        let p_offspring = 2.0 * (*s as f64) * p;
        total += p_offspring;
    }
    total
}
