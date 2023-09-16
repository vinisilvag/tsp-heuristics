use std::time::Instant;

mod args;

fn main() {
    let start = Instant::now();

    let distances: Vec<Vec<f32>> = args::read_args();

    println!("{:?}", distances);

    // call christofides solver

    let duration = start.elapsed();

    println!("Duration: {:?}", duration);

    // return results
}
