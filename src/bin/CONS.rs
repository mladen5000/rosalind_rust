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
    let reader = File::open("/Users/mladenrasic/Downloads/rosalind_cons.txt")
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .expect("Failed to open file and create reader");

    let (mut count_a, mut count_c, mut count_g, mut count_t) = (0, 0, 0, 0);
    // Go through the records and do boring stuff to get sequence
    for record in reader.records() {
        let record = record.expect("Failed to parse record");
        let _id = record.id().to_string();
        let seq = unsafe { String::from_utf8_unchecked(record.seq().to_owned()) }; // Clone the sequence data
        count_a = 0;
        count_c = 0;
        count_g = 0;
        count_t = 0;

        // Go through the chars
        for c in seq.chars() {
            match c {
                'A' | 'a' => {
                    count_a += 1;
                }
                'C' | 'c' => {
                    count_c += 1;
                }
                'G' | 'g' => {
                    count_g += 1;
                }
                'T' | 't' => {
                    count_t += 1;
                }
                _ => todo!(),
            }
        }
        println!("A: {}", count_a);
        println!("C: {}", count_c);
        println!("G: {}", count_g);
        println!("T: {}", count_t);
        println!();
        // Need to get count a at position 1, position 2, position 3, etc.
        // Then get the max of each position
        // Then I can do the same for each letter
    }
}
