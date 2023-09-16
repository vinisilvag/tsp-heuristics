mod algorithms;
mod args;

use std::time::{Duration, Instant};

fn main() {
    let (distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    let solution = algorithms::twice_around_the_tree(distances, graph);

    let duration: Duration = start.elapsed();

    println!("Solution: {}", solution);
    println!("Duration: {:?}", duration);
}
