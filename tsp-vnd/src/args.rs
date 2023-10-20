use clap::Parser;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

use std::path::Path;
use tspf::TspBuilder;

#[derive(Parser)]
#[command(
    author = "Vinicius Gomes",
    version,
    about = "A very simple CLI to execute a VND solution for instances of the Metric Traveling Salesman Problem in the TSP-LIB format"
)]
struct Arguments {
    #[arg(short, long)]
    /// The path for a TSP-LIB instance
    path: String,
}

fn euclidean(p0: (f64, f64), p1: (f64, f64)) -> f64 {
    let x: f64 = p1.0 - p0.0;
    let y: f64 = p1.1 - p0.1;

    ((x * x) + (y * y)).sqrt()
}

fn pseudo_euclidean(p0: (f64, f64), p1: (f64, f64)) -> f64 {
    let x: f64 = p1.0 - p0.0;
    let y: f64 = p1.1 - p0.1;

    let mut distance: f64 = (((x * x) + (y * y)) / 10.0).sqrt();
    let approx = distance.round();

    if approx < distance {
        distance = approx + 1.0;
    } else {
        distance = approx;
    }

    distance
}

pub fn read_args() -> (
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
    let weight_type = instance.weight_kind();

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
            let distance: f64 = match weight_type {
                tspf::WeightKind::Euc2d => euclidean(coords[i], coords[j]),
                tspf::WeightKind::Att => pseudo_euclidean(coords[i], coords[j]),
                _ => panic!("Invalid instance"),
            };

            distances[i][j] = distance;
            distances[j][i] = distance;

            graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), distances[i][j]);
        }
    }

    (name.to_string(), distances, graph)
}
