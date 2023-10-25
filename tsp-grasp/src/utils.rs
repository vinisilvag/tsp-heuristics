pub fn min(n1: f64, n2: f64) -> f64 {
    if n1 < n2 {
        n1
    } else {
        n2
    }
}

pub fn max(n1: f64, n2: f64) -> f64 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

pub fn two_opt_swap(
    path: &Vec<usize>,
    distances: &Vec<Vec<f64>>,
    cost: f64,
    u: usize,
    v: usize,
) -> (Vec<usize>, f64) {
    let mut opt_path = path.clone();
    let mut opt_cost = cost;

    opt_path[(u + 1)..=v].reverse();

    opt_cost -= distances[path[u]][path[u + 1]] + distances[path[v]][path[v + 1]];
    opt_cost += distances[path[u]][path[v]] + distances[path[u + 1]][path[v + 1]];

    (opt_path, opt_cost)
}
