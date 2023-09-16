use std::time::Instant;

use petgraph::Graph;

mod args;
mod solver;

fn main() {
    let graph: Graph<usize, f32, petgraph::Undirected> = args::read_args();

    let start = Instant::now();

    solver::christofides(graph);

    let duration = start.elapsed();

    println!("Duration: {:?}", duration);
}
