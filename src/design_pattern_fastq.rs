use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// This code utilizes both the builder and the iterator design pattern to parse a fastq file
// Product
#[derive(Debug)]
pub struct FastqRecord {
    id: String,
    seq: String,
    plus: String,
    qual: String,
}
impl Display for FastqRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n",
            self.id, self.seq, self.plus, self.qual
        )
    }
}

// Builder Object and Logic
struct FastqRecordBuilder {
    id: Option<String>,
    seq: Option<String>,
    plus: Option<String>,
    qual: Option<String>,
}
impl FastqRecordBuilder {
    fn new() -> Self {
        FastqRecordBuilder {
            id: None,
            seq: None,
            plus: None,
            qual: None,
        }
    }
    fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
    fn sequence(mut self, seq: String) -> Self {
        self.seq = Some(seq);
        self
    }
    fn plus(mut self, plus: String) -> Self {
        self.plus = Some(plus);
        self
    }
    fn qual(mut self, qual: String) -> Self {
        self.qual = Some(qual);
        self
    }
    fn build(self) -> Result<FastqRecord, &'static str> {
        if let (Some(id), Some(seq), Some(plus), Some(qual)) =
            (self.id, self.seq, self.plus, self.qual)
        {
            Ok(FastqRecord {
                id,
                seq,
                plus,
                qual,
            })
        } else {
            Err("Couldn't build the FastqRecord")
        }
    }
}

#[derive(Debug)]
pub struct FastqIterator<R: BufRead> {
    buffer: R,
}
impl<R: BufRead> FastqIterator<R> {
    fn new(reader: R) -> Self {
        Self { buffer: reader }
    }
}
impl<R: BufRead> Iterator for FastqIterator<R> {
    type Item = FastqRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut id, mut seq, mut plus, mut qual) =
            (String::new(), String::new(), String::new(), String::new());

        // Read buffer info into variables
        if [
            self.buffer.read_line(&mut id),
            self.buffer.read_line(&mut seq),
            self.buffer.read_line(&mut plus),
            self.buffer.read_line(&mut qual),
        ]
        .iter()
        .all(|result| result.is_ok() && result.as_ref().unwrap() > &0usize)
        {
            let record = FastqRecordBuilder::new()
                .id(id)
                .sequence(seq)
                .plus(plus)
                .qual(qual)
                .build()
                .ok();

            // let record = FastqRecordBuilder::new()
            //     .id(id.strip_prefix("@")?.trim().to_string())
            //     .sequence(seq.trim().to_string())
            //     .plus(plus.trim().to_string())
            //     .qual(qual.trim().to_string())
            //     .build()
            //     .ok();
            record
        } else {
            None
        }
    }
}

trait ParseStrategy<R: BufRead> {
    fn parse_reads(&self, buffer: FastqIterator<R>);
}

struct ParseCollect;
impl<R: BufRead> ParseStrategy<R> for ParseCollect {
    // Just collecting the items, no print. 0.88s
    fn parse_reads(&self, parser: FastqIterator<R>) {
        let len_items = parser.collect::<Vec<_>>().len();
        println!("{len_items}");
    }
}

struct ParseFold;
impl<R: BufRead> ParseStrategy<R> for ParseFold {
    // Read tracking implementation, 0.517s
    fn parse_reads(&self, parser: FastqIterator<R>) {
        let _count = parser.fold(0, |acc, _| {
            let count = acc + 1;
            if count % 10000 == 0 {
                println!("Processed {} sequences", count);
            }
            count
        });
    }
}

struct ParseParallel;
impl<R: BufRead> ParseStrategy<R> for ParseParallel {
    // Parallel Implementaton with print 16.38s, without, 0.6
    fn parse_reads(&self, parser: FastqIterator<R>) {
        let records: Vec<FastqRecord> = parser.collect();
        records
            .into_par_iter() // Takes ownership, but is faster than par_iter
            .for_each(|_record| ());
    }
}

struct ParseForLoop;
impl<R: BufRead> ParseStrategy<R> for ParseForLoop {
    // For loop implementaton, yes print. 27.63s
    fn parse_reads(&self, parser: FastqIterator<R>) {
        for (i, record) in parser.enumerate() {
            println!("{}:{}", i, record)
        }
    }
}

struct ParseWhile;
impl<R: BufRead> ParseStrategy<R> for ParseWhile {
    // While let implementation, 26.65s
    fn parse_reads(&self, mut parser: FastqIterator<R>) {
        // let mut parser = FastqIterator::new(fastq_buffer);
        while let Some(record) = parser.next() {
            println!("{}", record);
        }
    }
}
struct Parser<R: BufRead> {
    strategy: Box<dyn ParseStrategy<R>>,
}
fn main() -> Result<(), std::io::Error> {
    let path = "/Users/mladenrasic/Projects/WEVOTE-strain/testcov_R1.fastq";

    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let fq_iterator = FastqIterator::new(fastq_buffer);
    ParseCollect {}.parse_reads(fq_iterator);

    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let fq_iterator = FastqIterator::new(fastq_buffer);
    ParseFold {}.parse_reads(fq_iterator);

    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let fq_iterator = FastqIterator::new(fastq_buffer);
    ParseParallel {}.parse_reads(fq_iterator);

    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let fq_iterator = FastqIterator::new(fastq_buffer);
    ParseForLoop {}.parse_reads(fq_iterator);

    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let fq_iterator = FastqIterator::new(fastq_buffer);
    ParseWhile {}.parse_reads(fq_iterator);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastq_parser() {
        let fastq_content = "@SEQ1\nACTG\n+\nIIII\n@SEQ2\nTGCA\n+\nIIII";
        let buf = BufReader::new(Cursor::new(fastq_content));
        let mut parser = FastqIterator { buffer: buf };

        let record1 = parser.next().unwrap();
        assert_eq!(record1.id, "SEQ1");
        assert_eq!(record1.seq, "ACTG");
        assert_eq!(record1.plus, "+");
        assert_eq!(record1.qual, "IIII");

        let record2 = parser.next().unwrap();
        assert_eq!(record2.id, "SEQ2");
        assert_eq!(record2.seq, "TGCA");
        assert_eq!(record2.plus, "+");
        assert_eq!(record2.qual, "IIII");
        assert!(!parser.next().is_none());
    }

    #[test]
    fn test_fastq_parser_empty() {
        let fastq_content = "";
        let buf = BufReader::new(Cursor::new(fastq_content));
        let mut parser = FastqIterator { buffer: buf };

        assert!(parser.next().is_none());
    }
    #[test]
    fn test_fastq_parser_missing_fields() {
        let fastq_content = "@SEQ1\nACTG\n\n\n@SEQ2\nTGCA\n+\nIIII";
        let buf = BufReader::new(Cursor::new(fastq_content));
        let mut parser = FastqIterator { buffer: buf };

        let record1 = parser.next().unwrap();
        assert_eq!(record1.id, "SEQ1");
        assert_eq!(record1.seq, "ACTG");
        assert_eq!(record1.plus, "");
        assert_eq!(record1.qual, "");

        let record2 = parser.next().unwrap();
        assert_eq!(record2.id, "SEQ2");
        assert_eq!(record2.seq, "TGCA");
        assert_eq!(record2.plus, "+");
        assert_eq!(record2.qual, "IIII");
    }
}
