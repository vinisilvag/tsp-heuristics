pub fn adjacent_swap(
    path: &Vec<usize>,
    distances: &Vec<Vec<f64>>,
    cost: f64,
    u: usize,
) -> (Vec<usize>, f64) {
    let mut opt_path = path.clone();
    let mut opt_cost = cost;

    opt_path[u..=u + 1].reverse();

    opt_cost -= distances[path[u - 1]][path[u]] + distances[path[u + 1]][path[u + 2]];
    opt_cost += distances[path[u - 1]][path[u + 1]] + distances[path[u]][path[u + 2]];

    (opt_path, opt_cost)
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

pub fn or_opt_swap(
    path: &Vec<usize>,
    distances: &Vec<Vec<f64>>,
    cost: f64,
    u: usize,
    v: usize,
) -> (Vec<usize>, f64) {
    let mut opt_path = path.clone();
    let mut opt_cost = cost;

    opt_path[(u)..=v].reverse();

    opt_cost -= distances[path[u - 1]][path[u]] + distances[path[v]][path[v + 1]];
    opt_cost += distances[path[u - 1]][path[v]] + distances[path[u]][path[v + 1]];

    (opt_path, opt_cost)
}
