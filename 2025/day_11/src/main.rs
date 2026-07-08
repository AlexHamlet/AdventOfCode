use std::fs::read_to_string;

use petgraph::{Graph};

fn main() {
    let text_file_string =
        read_to_string("./day_11/input_sample.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().lines();
    let part_1_output = part_1(sample_input.clone());
    println!("Part 1: {part_1_output}");
    let part_2_output = part_2(sample_input);
    println!("Part 2: {part_2_output}");
}

fn part_1<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    let mut pathcount = 0;

    let mut graph = Graph::<&str, &str>::new();

    for line in lines{
        let (vertex, edges) = parse_line(line);
        // println!("Vertex {vertex}");
        // println!("Edges {:?}", edges);
        let a = graph.add_node(vertex);
        for edge in edges{
            let b = graph.coin(edge);
            graph.add_edge(a, b, edge);
        }
    }
    dbg!(graph);
    pathcount
}

fn part_2<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    0
}

fn parse_line(input: &str) -> (&str, Vec<&str>){
    let (vertex, remainder) = input.split_once(": ").expect("SplitOnce failed");
    let edges: Vec<&str> = remainder.split(" ").collect();
    (vertex, edges)
}