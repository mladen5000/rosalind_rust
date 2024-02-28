use std::{collections::BTreeMap, fs};

fn build_weights() -> BTreeMap<char, f64> {
    let mut map: BTreeMap<char, f64> = BTreeMap::new();
    let weights = [
        ('A', 71.03711),
        ('C', 103.00919),
        ('D', 115.02694),
        ('E', 129.04259),
        ('F', 147.06841),
        ('G', 57.02146),
        ('H', 137.05891),
        ('I', 113.08406),
        ('K', 128.09496),
        ('L', 113.08406),
        ('M', 131.04049),
        ('N', 114.04293),
        ('P', 97.05276),
        ('Q', 128.05858),
        ('R', 156.10111),
        ('S', 87.03203),
        ('T', 101.04768),
        ('V', 99.06841),
        ('W', 186.07931),
        ('Y', 163.06333),
    ];
    weights.iter().for_each(|&(key, value)| {
        map.insert(key, value).unwrap_or(0.0);
    });
    map
}

fn main() {
    let map = build_weights();
    let protein_string = "SKADYEK";

    let weight: f64 = protein_string.chars().filter_map(|aa| map.get(&aa)).sum();
    println!("{}", weight)
}
