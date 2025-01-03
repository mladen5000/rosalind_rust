use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

// Define a struct to hold a FASTQ record as borrowed slices
pub struct FastqRecord {
    pub id: String,
    pub sequence: String,
    pub plus: String,
    pub quality: String,
}

pub fn read_fastq_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<FastqRecord>> {
    // Open the FASTQ file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Process each line in the file
    let mut lines = reader.lines();
    let mut records = Vec::new();
    while let Some(Ok(id)) = lines.next() {
        let sequence = lines.next().expect("Invalid FASTQ file")?;
        let plus = lines.next().expect("Invalid FASTQ file")?;
        let quality = lines.next().expect("Invalid FASTQ file")?;

        // Create a FastqRecord instance
        let record = FastqRecord {
            id: id,
            sequence: sequence,
            plus: plus,
            quality: quality,
        };

        records.push(record);
    }

    Ok(records)
}
