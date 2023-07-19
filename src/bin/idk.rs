use std::fs::File;
use std::io::{self, Read};
use std::iter;

fn open_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    for line in &content {
        let line = line?;
    }

    println!("File content: {}", content);

    Ok(())
}
fn main() {
    let mypath = "./gc_content.rs";
    let _ = open_file(mypath);
}
