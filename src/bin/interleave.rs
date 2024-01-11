use bio::io::fastq::{self, Record};
use std::fmt::Error;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() {
    // case 1
    // let file1 = "/Users/mladenrasic/Downloads/sample.fastq";
    // let file2 = "/Users/mladenrasic/Downloads/sample.fastq";

    // case 2
    let file1 = "/Users/mladenrasic/Projects/rosalind_rust/MBTUMA001_S1_L001_R1_001.fastq";
    let file2 = "/Users/mladenrasic/Projects/rosalind_rust/MBTUMA001_S1_L001_R2_001.fastq";

    // interleave(file1, file2);
    merge(file1, file2).expect("No luck with merge");
}

fn interleave(file1: &str, file2: &str) {
    // Get readers and writers
    let reader1 = File::open(file1)
        .map(fastq::Reader::new)
        .expect("Can't open fastq file1");
    let reader2 = File::open(file2)
        .map(fastq::Reader::new)
        .expect("Can't open fastq file2");
    let mut writer = fastq::Writer::new(io::stdout());

    for (result1, result2) in reader1.records().zip(reader2.records()) {
        let record1 = result1.expect("Error during fastq record1 parsing");
        let record2 = result2.expect("Error during fastq record2 parsing");

        // Interleave
        writer
            .write_record(&record1)
            .expect("Record could not be written");
        writer
            .write_record(&record2)
            .expect("Record could not be written");
    }
}

fn merge(file1: &str, file2: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get readers and writers
    let reader1 = File::open(file1)
        .map(fastq::Reader::new)
        .expect("Can't open fastq file1");
    let reader2 = File::open(file2)
        .map(fastq::Reader::new)
        .expect("Can't open fastq file2");

    let writer_path = Path::new(file1)
        .parent()
        .expect("Source path has no parent.")
        .join("outfile.fastq");
    let mut writer = File::create(writer_path)
        .map(fastq::Writer::new)
        .expect("Can't create writer for writer_path");

    // let mut writer = fastq::Writer::new(io::stdout());
    // let mut writer = File::open()

    for (result1, result2) in reader1.records().zip(reader2.records()) {
        // Check for records
        let record1 = result1.expect("Result1 to record1 failed");
        let record2 = result2.expect("Result2 to record2 failed");

        // Name of each record should be the same with different desc()
        assert_eq!(
            record1.id(),
            record2.id(),
            "Record1: {:?} \n Record2: {:?}",
            record1.id(),
            record2.id()
        );

        // New merged record
        let new_id = record1.id();
        let new_seq = [record1.seq(), record2.seq()].concat();
        let new_qual = [record1.qual(), record2.qual()].concat();
        let new_record = Record::with_attrs(&new_id, None, new_seq.as_slice(), new_qual.as_slice());

        // Write record to stdout
        writer.write_record(&new_record)?;
        println!();
    }

    Ok(())
}
