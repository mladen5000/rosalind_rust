/*
>Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT
*/

/*
ATGCAACT
A: 5 1 0 0 5 5 0 0
C: 0 0 1 4 2 0 6 1
G: 1 1 6 3 0 1 0 0
T: 1 5 0 0 0 1 1 6
*/

use bio::io::fasta;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Parse the FASTQ file
    let reader = File::open("/Users/mladenrasic/Downloads/rosalind_cons-1.txt")
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .expect("Failed to open file and create reader");

    let mut matrix = vec![];

    // Go through the records and do boring stuff to get sequence
    for record in reader.records() {
        let record = record.expect("Failed to parse record");
        // let _id = record.id().to_string();
        let seq = unsafe { String::from_utf8_unchecked(record.seq().to_owned()) }; // Clone the sequence data

        // Populate consensus matrix
        for (i, c) in seq.chars().enumerate() {
            if matrix.len() < i + 1 {
                matrix.push(vec![0; 4])
            }
            match c {
                'A' | 'a' => matrix[i][0] += 1,
                'C' | 'c' => matrix[i][1] += 1,
                'G' | 'g' => matrix[i][2] += 1,
                'T' | 't' => matrix[i][3] += 1,
                _ => panic!("AHhhhhhhHHHHh"),
            }
        }
    }

    let consensus_seq = get_consensus_sequence(&matrix);
    println!("{consensus_seq}\n");
    // Output consense matrix
    print_consensus_matrix(&matrix);

    // println!("{:?}", matrix);
}

fn get_consensus_sequence(matrix: &Vec<Vec<i32>>) -> String {
    // Get the index where the max occurs for each col
    let consensus_index: Vec<_> = matrix
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .max_by_key(|&(_, item)| item)
                .unwrap()
                .0
        })
        .collect();
    // print!("{:?} ", consensus_index);

    let consensus_seq: String = consensus_index
        .iter()
        .map(|i| match i {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            3 => 'T',
            _ => unreachable!(),
        })
        .collect();
    // println!("{:?}", consensus_seq);
    consensus_seq
}

fn print_consensus_matrix(matrix: &Vec<Vec<i32>>) {
    for c in 0..matrix[0].len() {
        for r in 0..matrix.len() {
            print!("{} ", matrix[r][c]);
        }
        println!();
    }
}
