use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::visit::Dfs;

use crate::utils;

pub fn twice_around_the_tree(
    distances: &Vec<Vec<f64>>,
    graph: Graph<usize, f64, petgraph::Undirected>,
) -> (Vec<usize>, f64) {
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));

    let mut path: Vec<usize> = Vec::new();

    let mut dfs = Dfs::new(&mst, NodeIndex::new(0));
    while let Some(visited) = dfs.next(&mst) {
        path.push(visited.index());
    }

    path.push(0);

    let mut cost: f64 = 0.0;

    for i in 0..(path.len() - 1) {
        cost += distances[path[i]][path[i + 1]];
    }

    (path, cost)
}

pub fn variable_neighborhood_descent(
    distances: Vec<Vec<f64>>,
    graph: Graph<usize, f64, petgraph::Undirected>,
) -> (Vec<usize>, f64) {
    let n = graph.node_count();
    let (mut curr, mut best) = twice_around_the_tree(&distances, graph);

    let mut is_improving = true;

    while is_improving {
        is_improving = false;

        // ADJACENT-SWAP
        for u in 1..(n - 2) {
            let (opt_path, opt_cost) = utils::adjacent_swap(&curr, &distances, best, u);

            if best > opt_cost {
                curr = opt_path;
                best = opt_cost;
                is_improving = true;
            }
        }

        // 2-OPT
        for u in 1..(n - 2) {
            for v in (u + 1)..(n - 1) {
                let (opt_path, opt_cost) = utils::two_opt_swap(&curr, &distances, best, u, v);

                if best > opt_cost {
                    curr = opt_path;
                    best = opt_cost;
                    is_improving = true;
                }
            }
        }

        // 3-OPT
        for u in 1..(n - 3) {
            for v in (u + 1)..(n - 2) {
                for w in (v + 1)..(n - 1) {
                    let (opt_path, opt_cost) =
                        utils::three_opt_swap(&curr, &distances, best, u, v, w);

                    if best > opt_cost {
                        curr = opt_path;
                        best = opt_cost;
                        is_improving = true;
                    }
                }
            }
        }

        // 4-OPT
        for u in 1..(n - 4) {
            for v in (u + 1)..(n - 3) {
                for w in (v + 1)..(n - 2) {
                    for z in (w + 1)..(n - 1) {
                        let (opt_path, opt_cost) =
                            utils::four_opt_swap(&curr, &distances, best, u, v, w, z);

                        if best > opt_cost {
                            curr = opt_path;
                            best = opt_cost;
                            is_improving = true;
                        }
                    }
                }
            }
        }
    }

    (curr, best)
}
