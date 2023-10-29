use petgraph::graph::Graph;
use rand::Rng;

use crate::utils;

fn greedy_randomized_construction(
    alfa: f64,
    n: usize,
    distances: &Vec<Vec<f64>>,
) -> (Vec<usize>, f64) {
    let mut rng = rand::thread_rng();

    let mut solution = Vec::<usize>::new();
    let mut cost = 0.0;
    let mut visited = vec![false; n];

    let start = rng.gen_range(0..n);

    solution.push(start);
    visited[start] = true;

    while solution.len() < n {
        let last_node = solution[solution.len() - 1];
        let mut costs = Vec::<(usize, f64)>::new();

        let mut c_min = f64::MAX;
        let mut c_max = 0.0;

        for i in 0..n {
            if visited[i] == false {
                let cost = distances[last_node][i];

                c_min = utils::min(c_min, cost);
                c_max = utils::max(c_max, cost);

                costs.push((i, cost));
            }
        }

        let mut rcl = Vec::<(usize, f64)>::new();

        for (index, cost) in costs {
            let restriction = c_min + alfa * (c_max - c_min);

            if cost <= restriction {
                rcl.push((index, cost));
            }
        }

        let node = rcl[rng.gen_range(0..rcl.len())];

        solution.push(node.0);
        visited[node.0] = true;

        cost += node.1;
    }

    solution.push(start);
    cost += distances[solution[solution.len() - 2]][solution[solution.len() - 1]];

    (solution, cost)
}

fn local_search(
    n: usize,
    distances: &Vec<Vec<f64>>,
    curr: Vec<usize>,
    best: f64,
) -> (Vec<usize>, f64) {
    let mut solution = curr.clone();
    let mut cost = best;

    let mut is_improving = true;

    while is_improving {
        is_improving = false;

        // 2-OPT
        for u in 0..(n - 1) {
            for v in (u + 1)..n {
                let (opt_path, opt_cost) = utils::two_opt_swap(&solution, &distances, cost, u, v);

                if opt_cost < cost {
                    solution = opt_path;
                    cost = opt_cost;
                    is_improving = true;
                }
            }
        }
    }

    (solution, cost)
}

pub fn grasp(
    distances: Vec<Vec<f64>>,
    graph: Graph<usize, f64, petgraph::Undirected>,
    alfa: f64,
    max_iter: i32,
) -> (Vec<usize>, f64) {
    let n = graph.node_count();

    let mut best_solution: Vec<usize> = Vec::new();
    let mut best_cost: f64 = f64::MAX;

    for _ in 0..max_iter {
        let (initial_path, initial_cost) = greedy_randomized_construction(alfa, n, &distances);

        let (path, cost) = local_search(n, &distances, initial_path, initial_cost);

        if cost < best_cost {
            best_solution = path;
            best_cost = cost;
        }
    }

    (best_solution, best_cost)
}
