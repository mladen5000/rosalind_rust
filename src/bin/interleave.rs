use std::fs::File;
use std::io::{BufRead, BufReader, Write};
extern crate bio;
use bio::io::fastq::{self, Error};

fn interleave_fastq(
    reader1: fastq::Reader<BufReader<File>>,
    reader2: fastq::Reader<BufReader<File>>,
    writer: &mut fastq::Writer<File>,
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
        let merge_seq = format!("{:?}{:?}", r1.seq(), r2.seq());
        let merge_qual = format!("{:?}{:?}", r1.qual(), r2.qual());
        let new_record = fastq::Record::with_attrs(
            r1.id(),
            r1.desc(),
            merge_seq.as_bytes(),
            merge_qual.as_bytes(),
        );

        writer.write_record(&new_record)?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let file1 = File::open("/Users/mladenrasic/Downloads/sample.fastq")?;
    let file2 = File::open("/Users/mladenrasic/Downloads/sample.fastq")?;
    let out_file = File::create("/Users/mladenrasic/Downloads/interleave.fastq")?;

    let reader1 = fastq::Reader::new(file1);
    let reader2 = fastq::Reader::new(file2);
    let mut writer = fastq::Writer::new(out_file);

    // interleave_fastq2(reader1, reader2, &mut writer)?;
    merge_reads(reader1, reader2, &mut writer)?;
    Ok(())
}
