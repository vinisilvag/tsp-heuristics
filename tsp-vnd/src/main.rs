mod algorithms;
mod args;
mod utils;

use std::time::{Duration, Instant};

fn main() {
    let (_, distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    let (path, cost): (Vec<usize>, f64) =
        algorithms::variable_neighborhood_descent(distances, graph);

    let duration: Duration = start.elapsed();

    println!("Cost: {} - Duration: {:?}", cost, duration);
    println!("Path: {:?}", path);
}
