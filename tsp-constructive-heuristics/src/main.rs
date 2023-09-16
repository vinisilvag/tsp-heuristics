use std::time::{Duration, Instant};

mod args;
mod solvers;

fn main() {
    let (distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    let solution = solvers::christofides(distances, graph);

    let duration: Duration = start.elapsed();

    println!("Solution: {}", solution);
    println!("Duration: {:?}", duration);
}
