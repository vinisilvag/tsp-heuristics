mod algorithms;
mod args;

use std::time::{Duration, Instant};

fn main() {
    let (_, distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    let (path, cost): (Vec<usize>, f64) = algorithms::twice_around_the_tree(distances, graph);

    let duration: Duration = start.elapsed();

    println!("Cost: {} - Duration: {:?}", cost, duration);
    println!("Path: {:?}", path);
}
