use clap::Parser;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(clap::ValueEnum, Clone)]
pub enum Algorithm {
    TwiceAroundTheTree,
    Christofides,
}

#[derive(Parser)]
#[command(
    author = "Vinicius Gomes",
    version,
    about = "A very simple CLI to execute two constructive heuristics (Twice Around the Tree and Christofides) for instances of the Metric Traveling Salesman Problem"
)]
struct Arguments {
    #[arg(short, long)]
    /// The heuristic algorithm that is going to be executed
    algorithm: Algorithm,

    #[arg(short, long)]
    /// The path for a TSP-LIB instance
    path: String,
}

fn open_file(path: String) -> String {
    let mut file = File::open(path).expect("Can't open this entry!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Cannot read the file!");

    contents
}

fn get_entry_data(contents: String) -> (usize, String) {
    let mut size: usize = 0;
    let mut points: String = String::new();

    let mut re = Regex::new(r"DIMENSION\s*: (.+)").unwrap();

    if let Some(capture) = re.captures(&contents) {
        if let Some(dimension) = capture.get(1) {
            size = dimension.as_str().parse().unwrap();
        }
    }

    re = Regex::new(r"NODE_COORD_SECTION\n([\s\S]*?)\nEOF").unwrap();
    if let Some(capture) = re.captures(&contents) {
        if let Some(section) = capture.get(1) {
            points = section.as_str().to_string();
        }
    }

    (size, points)
}

fn euclidean(p0: (f32, f32), p1: (f32, f32)) -> f32 {
    let x: f32 = p1.0 - p0.0;
    let y: f32 = p1.1 - p0.1;

    ((x * x) + (y * y)).sqrt()
}

pub fn read_args() -> (
    Algorithm,
    Vec<Vec<f32>>,
    Graph<usize, f32, petgraph::Undirected>,
) {
    let args = Arguments::parse();

    let contents: String = open_file(args.path);

    let (size, points) = get_entry_data(contents);

    let mut coords = Vec::new();

    for line in points.lines() {
        let node: Vec<&str> = line.split_whitespace().collect();

        let x: f32 = node[1].parse().unwrap();
        let y: f32 = node[2].parse().unwrap();

        coords.push((x, y));
    }

    let mut graph = Graph::<usize, f32, petgraph::Undirected>::new_undirected();

    let mut distances = vec![vec![-1.0f32; size]; size];

    for i in 0..size {
        graph.add_node(i);
    }

    for i in 0..size {
        for j in 0..i {
            let distance = euclidean(coords[i], coords[j]);

            distances[i][j] = distance;
            distances[j][i] = distance;

            graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), distances[i][j]);
        }
    }

    (args.algorithm, distances, graph)
}
