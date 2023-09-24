use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::visit::Dfs;

pub fn twice_around_the_tree(
    distances: Vec<Vec<f64>>,
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
