use rayon::prelude::*;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

// This code utilizes both the builder and the iterator design pattern to parse a fastq file
// Product
#[derive(Debug)]
struct FastqRecord {
    id: String,
    seq: String,
    plus: String,
    qual: String,
}
impl Display for FastqRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "@{}\n{}\n{}\n{}\n\n",
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
    lines: R,
}
impl<R: BufRead + Send> FastqIterator<R> {
    fn new(lines: R) -> Self {
        FastqIterator { lines }
    }
}
impl<R: BufRead + Send> Iterator for FastqIterator<R> {
    type Item = FastqRecord;

    fn next(&mut self) -> Option<Self::Item> {
        // Initalize the variables
        let (mut id, mut seq, mut qual) = (String::new(), String::new(), String::new());

        // Read buffer info into variables
        if let (Ok(_), Ok(_), Ok(_)) = (
            self.lines.read_line(&mut id),
            self.lines.read_line(&mut seq),
            self.lines.read_line(&mut qual),
        ) {
            // Return the class
            FastqRecordBuilder::new()
                .id(id.trim_start_matches('@').trim().to_string())
                .sequence(seq.trim().to_string())
                .plus('+'.to_string())
                .qual(qual.trim().to_string())
                .build()
                .ok()
        } else {
            None // Return None for End of Iteration
        }
    }
}
impl<'a, R: BufRead + Read> IntoParallelIterator for &'a FastqIterator<R>
where
    R: Send + Sync,
    R::Error: Send + Sync,
    <Self as Iterator>::Item: Send + Sync,
{
    type Item = FastqRecord;
    type Iter = rayon::iter::Cloned<rayon::slice::Iter<'a, FastqRecord>>;
}
fn main() -> Result<(), std::io::Error> {
    let path = Path::new("/Users/mladenrasic/Projects/WEVOTE-strain/testcov_R1.fastq");
    let fastq_buffer = BufReader::new(File::open(path).expect("Failed to open file"));
    let parser = FastqIterator::new(fastq_buffer);
    // let records: Vec<FastqRecord> = parser.collect();

    println!("Parallel Time!");
    parser.par_iter().for_each(|record| println!("{}", record));

    // for record in parser {
    //     println!("{}", record)
    // }

    // while let Some(record) = parser.iter_records() {
    //     println!("{}", record);
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastq_parser() {
        let fastq_content = "@SEQ1\nACTG\n+\nIIII\n@SEQ2\nTGCA\n+\nIIII";
        let buf = BufReader::new(Cursor::new(fastq_content));
        let mut parser = FastqIterator { lines: buf };

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
        assert!(!parser.iter_records().is_none());
    }

    #[test]
    fn test_fastq_parser_missing_fields() {
        let fastq_content = "@SEQ1\nACTG\n+\nIIII\n@SEQ2\nTGCA\n+";
        let buf = BufReader::new(Cursor::new(fastq_content));
        let mut parser = FastqIterator(buf);
        assert!(!parser.iter_records().is_none());
    }
}
