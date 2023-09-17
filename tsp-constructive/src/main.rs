mod algorithms;
mod args;

use crate::args::Algorithm;
use std::time::{Duration, Instant};

fn main() {
    let (algorithm, distances, graph) = args::read_args();

    let start: Instant = Instant::now();

    // Twice Around the Tree and Christofides available
    let cost: f32 = match algorithm {
        Algorithm::Christofides => algorithms::christofides(distances, graph),
        Algorithm::TwiceAroundTheTree => algorithms::twice_around_the_tree(distances, graph),
    };

    let duration: Duration = start.elapsed();

    println!("Cost: {}", cost);
    println!("Duration: {:?}", duration);
}
