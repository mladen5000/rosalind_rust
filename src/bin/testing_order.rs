use core::fmt;
use std::{cmp::Ordering, fmt::Display};

use bio::seq_analysis;

struct Point {
    x: i32,
    y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // First compare by year
        if let Some(ordering) = self.x.partial_cmp(&other.x) {
            if ordering != Ordering::Equal {
                return Some(ordering);
            }
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by year
        match self.x.cmp(&other.x) {
            Ordering::Equal => (),
            ordering => return ordering,
        }
        self.y.cmp(&other.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        // Check if equals
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(PartialEq, Eq)]
struct FastqRecord {
    id: String,
    sequence: String,
    quality: String,
}
impl FastqRecord {
    fn new() -> FastqRecord {
        FastqRecord {
            id: String::new(),
            sequence: String::new(),
            quality: String::new(),
        }
    }
}

impl Clone for FastqRecord {
    fn clone(&self) -> Self {
        FastqRecord {
            id: self.id.clone(),
            sequence: self.sequence.clone(),
            quality: self.quality.clone(),
        }
    }
}

impl Display for FastqRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}\n{}\n+\n{}", self.id, self.sequence, self.quality)
    }
}
impl std::str::FromStr for FastqRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().ok_or("missing ID line")?;
        let sequence = lines.next().ok_or("missing sequence line")?;
        let quality_header = lines.next().ok_or("missing sequence header")?;
        if quality_header != "+" {
            return Err(format!("Invalid quality header {}", quality_header));
        }
        let quality = lines.next().ok_or("missing sequence line")?;
        Ok(FastqRecord {
            id: id.to_owned(),
            sequence: sequence.to_owned(),
            quality: quality.to_owned(),
        })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 5, y: 3 };
    let _point3 = Point { x: 1, y: 3 };
    let point4 = Point { x: 1, y: 2 };
    assert!(point1 < point2);
    assert!(point1 == point4);
    Ok(())
}
