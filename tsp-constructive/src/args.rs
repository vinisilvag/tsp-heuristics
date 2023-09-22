use clap::Parser;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

use std::path::Path;
use tspf::TspBuilder;

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

fn euclidean(p0: (f64, f64), p1: (f64, f64)) -> f64 {
    let x: f64 = p1.0 - p0.0;
    let y: f64 = p1.1 - p0.1;

    ((x * x) + (y * y)).sqrt()
}

pub fn read_args() -> (
    Algorithm,
    String,
    Vec<Vec<f64>>,
    Graph<usize, f64, petgraph::Undirected>,
) {
    let args = Arguments::parse();

    let instance = match TspBuilder::parse_path(Path::new(args.path.as_str())) {
        Ok(tsp) => tsp,
        Err(error) => panic!("Error reading the instance: {:?}", error),
    };

    let name = instance.name();
    let size: usize = instance.dim();

    let mut coords = Vec::new();

    for i in instance.node_coords() {
        let x: f64 = i.1.pos()[0];
        let y: f64 = i.1.pos()[1];

        coords.push((x, y));
    }

    let mut distances = vec![vec![-1.0f64; size]; size];

    let mut graph = Graph::<usize, f64, petgraph::Undirected>::new_undirected();

    // O(V)
    for i in 0..size {
        graph.add_node(i);
    }

    // O(E)
    for i in 0..size {
        for j in 0..i {
            let distance = euclidean(coords[i], coords[j]);

            distances[i][j] = distance;
            distances[j][i] = distance;

            graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), distances[i][j]);
        }
    }

    (args.algorithm, name.to_string(), distances, graph)
}
