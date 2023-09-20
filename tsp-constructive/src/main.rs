mod algorithms;
mod args;

use crate::args::Algorithm;
use std::time::{Duration, Instant};

fn main() {
    let (algorithm, distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    // [WIP] Christofides
    let (path, cost): (Vec<usize>, f32) = match algorithm {
        Algorithm::Christofides => algorithms::christofides(distances, graph),
        Algorithm::TwiceAroundTheTree => algorithms::twice_around_the_tree(distances, graph),
    };

    let duration: Duration = start.elapsed();

    println!("Cost: {} - Duration: {:?}", cost, duration);
    println!("Path: {:?}", path);
}
