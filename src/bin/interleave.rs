use std::io::{BufReader, Read};
use std::{fmt::Write, fs::File};
extern crate bio;
use bio::io::fastq::{self, Error, FastqRead};

/*
fn interleave_fastq<R: FastqRead, W: Write>(

    reader1: R,
    reader2: R,
    writer: &mut W,
) -> Result<(), Error> {
    let mut iter1 = reader1.records();
    let mut iter2 = reader2.records();

    loop {
        let record1 = iter1.next();
        let record2 = iter2.next();

        if record1.is_none() || record2.is_none() {
            break;
        }

        let record1 = record1.ok_or(0).unwrap()?;
        let record2 = record2.ok_or(0).unwrap()?;

        writer.write_record(&record1)?;
        writer.write_record(&record2)?;
    }

    Ok(())
}
*/

fn interleave_fastq2(
    reader1: fastq::Reader<BufReader<File>>,
    reader2: fastq::Reader<BufReader<File>>,
    writer: &mut fastq::Writer<File>,
) -> Result<(), Error> {
    let record1 = reader1.records();
    let record2 = reader2.records();

    for (r1, r2) in record1.zip(record2) {
        let r1 = r1?;
        let r2 = r2?;
        writer.write_record(&r1)?;
        writer.write_record(&r2)?;
    }

    Ok(())
}

pub fn merge_reads(
    reader1: fastq::Reader<BufReader<File>>,
    reader2: fastq::Reader<BufReader<File>>,
    writer: &mut fastq::Writer<File>,
) -> Result<(), Error> {
    let record1 = reader1.records();
    let record2 = reader2.records();

    for (r1, r2) in record1.zip(record2) {
        let r1 = r1?;
        let r2 = r2?;

        let concatenated_seq = format!(
            "{}{}",
            String::from_utf8_lossy(r1.seq()),
            String::from_utf8_lossy(r2.seq())
        );
        let concatenated_qual = format!(
            "{}{}",
            String::from_utf8_lossy(&r1.qual()),
            String::from_utf8_lossy(&r2.qual())
        );

        let new_record = fastq::Record::with_attrs(
            r1.id(),
            r1.desc(),
            concatenated_seq.as_bytes(),
            concatenated_qual.as_bytes(),
        );

        writer.write_record(&new_record)?;
    }

    Ok(())
}

pub fn filter_by_quality(
    reader1: fastq::Reader<BufReader<File>>,
    writer: &mut fastq::Writer<File>,
    min_score: f32,
) {
    let mut records = reader1.records();
    while let Some(Ok(record)) = records.next() {
        let qual = record.qual();
        let mean_quality = qual.iter().map(|&q| f32::from(q)).sum::<f32>() / qual.len() as f32;
        if mean_quality > min_score {
            writer.write_record(&record).unwrap();
        }
    }
}
fn main() -> Result<(), Error> {
    let file1 = File::open("/Users/mladenrasic/Downloads/sample.fastq")?;
    let file2 = File::open("/Users/mladenrasic/Downloads/sample.fastq")?;
    let out_file = File::create("/Users/mladenrasic/Downloads/interleave.fastq")?;

    let reader1 = fastq::Reader::new(file1);
    let reader2 = fastq::Reader::new(file2);
    let mut writer = fastq::Writer::new(out_file);

    // interleave_fastq2(reader1, reader2, &mut writer)?;
    // merge_reads(reader1, reader2, &mut writer)?;
    filter_by_quality(reader1, &mut writer, 50.0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_interleave_fastq() {
        // Create input FASTQ files
        let input1 = BufReader::new("@read1\nACGT\n+\n!!!!\n@read3\nCGTA\n+\n!!!!\n");
        let input2 = BufReader::new("@read2\nTGCA\n+\n!!!!\n@read4\nCATG\n+\n!!!!\n");

        // Create expected output FASTQ file
        let expected_output = "@read1\nACGT\n+\n!!!!\n@read2\nTGCA\n+\n!!!!\n@read3\nCGTA\n+\n!!!!\n@read4\nCATG\n+\n!!!!\n";

        // Create output buffer
        let mut output = Vec::new();

        // Call interleave_fastq function
        let reader1 = fastq::Reader::new(input1);
        let reader2 = fastq::Reader::new(input2);
        let mut writer = fastq::Writer::new(&mut output);
        interleave_fastq(reader1, reader2, &mut writer).unwrap();

        // Check output against expected output
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    #[test]
    fn test_interleave_fastq_empty() {
        // Create input FASTQ files
        let input1 = BufReader::new("");
        let input2 = BufReader::new("");

        // Create expected output FASTQ file
        let expected_output = "";

        // Create output buffer
        let mut output = Vec::new();

        // Call interleave_fastq function
        let reader1 = fastq::Reader::new(input1);
        let reader2 = fastq::Reader::new(input2);
        let mut writer = fastq::Writer::new(&mut output);
        interleave_fastq(reader1, reader2, &mut writer).unwrap();

        // Check output against expected output
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    #[test]
    fn test_interleave_fastq_unequal() {
        // Create input FASTQ files
        let input1 = File::from("@read1\nACGT\n+\n!!!!\n@read3\nCGTA\n+\n!!!!\n");
        let input2 = File::from("@read2\nTGCA\n+\n!!!!\n");

        // Create expected output FASTQ file
        let expected_output =
            "@read1\nACGT\n+\n!!!!\n@read2\nTGCA\n+\n!!!!\n@read3\nCGTA\n+\n!!!!\n";

        // Create output buffer
        let mut output = File::from();

        // Call interleave_fastq function
        let reader1 = fastq::Reader::new(input1);
        let reader2 = fastq::Reader::new(input2);
        let mut writer = fastq::Writer::new(&mut output);
        interleave_fastq(reader1, reader2, &mut writer).unwrap();

        // Check output against expected output
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }
}
