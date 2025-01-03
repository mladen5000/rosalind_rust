use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
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
struct OverlapGraph<'a> {
    nodes: &'a Vec<Node>,
    edges: &'a Vec<Edge>,
}

fn main() {
    let n = 3; //Overlap value
    let mut node_vec: Vec<Node> = vec![];
    let mut edge_vec: Vec<Edge> = vec![];

    // Parse the FASTQ file
    let reader = BufReader::new(
        File::open("/Users/mladenrasic/Downloads/rosalind_grph.txt").expect("Failed to open file"),
    );
    let parser = fasta::Reader::new(reader);

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
            id: id.to_string(), // Clone the id
            seq: seq.to_string(),
        };

        // Add the node to the vector
        node_vec.push(node);
    }
    let mut node_vec2 = node_vec.clone();

    // Determine all edges with an overlap of n
    for i in node_vec.iter_mut() {
        for j in node_vec2.iter_mut() {
            let suffix = &i.seq[(i.seq.len() - n)..i.seq.len()]; // get last 3 NTs
            let prefix = &j.seq[..n]; // get first 3 NTs

            // Sanity check
            assert_eq!(suffix.len(), n);
            assert_eq!(suffix.len(), prefix.len());

            // Determine overlap regions and build edge
            if suffix == prefix {
                // Append Edge onto the Vec<Edge>
                edge_vec.push(Edge {
                    from: (i).clone(),
                    to: (j).clone(),
                });
                println!("{} {}", i.id, j.id);
                // Clone the Node structs
                // edge_vec.push(edge);
            }
        }
    }
    let overlap_graph = OverlapGraph {
        // Create the OverlapGraph struct
        nodes: &node_vec,
        edges: &edge_vec,
    };
    println!(
        "{:?}\n{:?}",
        overlap_graph.nodes.len(),
        overlap_graph.edges.len()
    );

    // Process each record
    // println!("{:?}", full_vec);
}
