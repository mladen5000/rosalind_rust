use bio::io::fasta;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Node {
    id: String,
    seq: String,
}
#[derive(Debug)]
struct Edge {
    from: Node,
    to: Node,
}

fn main() {
    let n = 3; //Overlap value
    let mut node_vec: Vec<Node> = vec![];
    let mut edge_vec: Vec<Edge> = vec![];

    // Parse the FASTQ file
    let reader = File::open("/Users/mladenrasic/Downloads/rosalind_grph-3.txt")
        .map(BufReader::new)
        .map(fasta::Reader::new)
        .expect("Failed to open file and create reader");

    // Open the output file
    let mut output_file =
        File::create("/Users/mladenrasic/Projects/rosalind_rust/src/bin/output.txt")
            .expect("Failed to create output file");
    // Initialize the node
    let mut node = Node {
        id: String::new(),
        seq: String::new(),
    };

    // Build the Vec<Node>

    for record in reader.records() {
        let record = record.expect("Failed to parse record");
        let id = record.id().to_string();
        let seq = unsafe { String::from_utf8_unchecked(record.seq().to_owned()) }; // Clone the sequence data

        // Create a new node
        let node = Node {
            id: id.to_string(),
            seq: seq.to_string(),
        };

        // Add the node to the vector
        node_vec.push(node);
    }

    // Determine all edges with an overlap of n
    for i in 0..node_vec.len() {
        for j in 0..node_vec.len() {
            if i != j {
                let suffix_i = &node_vec[i].seq[node_vec[i].seq.len() - n..]; // get last 3 NTs
                let prefix_j = &node_vec[j].seq[..n]; // get first 3 NTs
                if suffix_i == prefix_j {
                    let edge = Edge {
                        from: node_vec[i].clone(),
                        to: node_vec[j].clone(),
                    };
                    // Write the edges to the file
                    let edge_str = format!("{} {}", &edge.from.id, &edge.to.id);
                    println!("{}", edge_str);
                    edge_vec.push(edge);
                }
            }
        }
    }
    println!("{:?}", edge_vec.len());
}
