use anyhow::{Context, Result};
use bio::io::fastq::{self, Reader, Record};
use std::{
    error::{self, Error},
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
// Import the missing Reader type

fn main() {
    // case 2
    let file1 = "/Users/mladenrasic/Projects/rosalind_rust/MBTUMA001_S1_L001_R1_001.fastq";
    let file2 = "/Users/mladenrasic/Projects/rosalind_rust/MBTUMA001_S1_L001_R2_001.fastq";

    let reader1 = get_reader(file1);
    let reader2 = get_reader(file2);
    // Run function
    process_reverse_complement(reader1);
    interleave(reader1, reader2);
    deinterleave(reader1);
    merge(reader1, reader2).expect("No luck with merge");
}

fn process_reverse_complement<R>(reader: Reader<BufReader<R>>)
where
    R: io::Read,
{
    let mut writer = fastq::Writer::new(io::stdout());

    for result in reader.records() {
        let record = result.expect("Can't get record from reader");
        let reverse_complement: String = record
            .seq()
            .iter()
            .rev()
            .map(|c| match *c as char {
                'A' | 'a' => 'T',
                'T' | 't' => 'A',
                'C' | 'c' => 'G',
                'G' | 'g' => 'C',
                x => x,
                _ => panic!("Invalid character"),
            })
            .collect::<String>(); // Collect the characters into a String

        // Build Record
        let rc_record = Record::with_attrs(
            &record.id(),
            None,
            reverse_complement.as_bytes(),
            record.qual(),
        );

        // Write record to stdout
        writer
            .write_record(&rc_record)
            .expect("Record could not be written");
    }
}

fn interleave<R>(reader1: Reader<BufReader<R>>, reader2: Reader<BufReader<R>>)
where
    R: io::Read,
{
    // Get readers and writers

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

fn deinterleave<R>(reader: Reader<BufReader<R>>) -> Result<()>
where
    R: io::Read,
{
    let mut writer1 = fastq::Writer::new(io::stdout());
    let mut writer2 = fastq::Writer::new(io::stdout());

    // Collect records into chunks of 2
    let nums: Vec<_> = reader1.records().collect();

    for chunk in nums.chunks(2) {
        let (result1, result2) = match chunk {
            [Ok(record1), Ok(record2)] => (record1, record2),
            _ => panic!("Invalid chunk size"),
        };

        let record_blank = Record::default();
        let record1 = result1; //.expect("nums");
        let record2 = result2; //.expect("nums2");
        println!();
        writer1
            .write_record(&record_blank)
            .expect("Can't write blank record");
        writer1
            .write_record(&record1)
            .expect("Record1 could not be written");
        writer2
            .write_record(&record2)
            .expect("Record2 could not be written");
        println!();
    }

    // Interleave
    Ok(())
}

fn merge<R>(reader1: Reader<BufReader<R>>, reader2: Reader<BufReader<R>>) -> Result<()>
where
    R: io::Read,
{
    // Get readers and writers

    let mut writer = fastq::Writer::new(io::stdout());

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

fn write_to_file() {
    let writer_path = Path::new("/Users/mladenrasic/")
        .parent()
        .expect("Source path has no parent.")
        .join("outfile.fastq");

    let mut writer = File::create(&writer_path)
        .map(fastq::Writer::new)
        .expect("Can't create writer for writer_path");

    // Write to the file
    let record = bio::io::fastq::Record::new();
    writer
        .write_record(&record)
        .expect("Failed to write to file");
}

fn get_reader<R>(file: &str) -> Reader<BufReader<R>>
where
    R: io::Read,
{
    let reader = File::open(file)
        .map(BufReader::new)
        .map(fastq::Reader::new)
        .expect("Can't open fastq file1");
    reader
}
