fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else if b.len() > a.len() {
        b
    } else {
        "Same length"
    }
}
struct BookReview {
    title: String,
    rating: u8, // Rating out of 10
}

// Function with multiple lifetimes
fn choose_title<'a>(title: &'a str, review: &'a BookReview, default_title: &'a str) -> &'a str {
    if review.rating > 8 {
        title
    } else {
        default_title
    }
}

fn main() {
    let book_title = "Rust Programming";
    let default_title = "Not Available";
    let review = BookReview {
        title: String::from("Rust Programming"),
        rating: 9,
    };

    let chosen_title = choose_title(book_title, &review, default_title);
    println!("Chosen title: {}", chosen_title);
}
