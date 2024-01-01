use std::cmp::Ordering;

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

fn main() -> Result<(), E> {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 5, y: 3 };
    let point3 = Point { x: 1, y: 3 };
    assert_eq!(point1, point2);
    assert_eq!(point2, point3);
    println!("Hi")
}
