use bio::io::fastq;
use needletail::parser;
use std::fs::File;
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

#[derive(Debug)]
struct OverlapGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn main() {
    let mut node_vec: Vec<Node> = vec![];
    let mut edge_vec: Vec<Edge> = vec![];

    // Parse the FASTQ file
    let reader = BufReader::new(
        File::open("/Users/mladenrasic/Downloads/sample.fastq").expect("Failed to open file"),
    );
    let parser = fastq::Reader::new(reader);

    // Initialize the node
    let mut node = Node {
        id: String::new(),
        seq: String::new(),
    };

    // Build the Vec<Node>
    for record in parser.records() {
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
    for i in node_vec.iter() {
        for j in node_vec.iter() {
            let suffix = &i.seq[(i.seq.len() - 3)..i.seq.len()]; // get last 3 NTs
            let prefix = &j.seq[..3]; // get first 3 NTs

            // Sanity check

            assert_eq!(suffix.len(), 3);
            assert_eq!(suffix.len(), prefix.len());
            if suffix == prefix {
                // println!("{}...{}", i.id, j.id);
                let edge = Edge {
                    from: (*i).clone(),
                    to: (*j).clone(),
                }; // Clone the Node structs
                edge_vec.push(edge);
            }
        }
    }
    let overlap_graph = OverlapGraph {
        nodes: node_vec,
        edges: edge_vec,
    };
    println!("{:?}", overlap_graph);

    // Process each record
    // println!("{:?}", full_vec);
}
