use petgraph::algo::{maximum_matching, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::Dot;
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

pub fn christofides(
    distances: Vec<Vec<f64>>,
    graph: Graph<usize, f64, petgraph::Undirected>,
) -> (Vec<usize>, f64) {
    println!("{:?}", Dot::new(&graph));

    let mut mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));

    println!("{:?}", Dot::new(&mst));

    let mut odd_nodes: Vec<NodeIndex> = Vec::new();

    for i in mst.node_indices() {
        let neighbors = mst.neighbors(i);
        let node_degree = neighbors.count();

        if node_degree % 2 != 0 {
            odd_nodes.push(i);
        }
    }

    let mut odd_subgraph = Graph::<usize, f64, petgraph::Undirected>::new_undirected();

    for i in 0..odd_nodes.len() {
        odd_subgraph.add_node(NodeIndex::index(odd_nodes[i]));
    }

    for i in 0..odd_nodes.len() {
        for j in (i + 1)..odd_nodes.len() {
            odd_subgraph.add_edge(
                NodeIndex::new(i),
                NodeIndex::new(j),
                distances[odd_nodes[i].index()][odd_nodes[j].index()],
            );
        }
    }

    println!("{:?}", Dot::new(&odd_subgraph));

    let max_match = maximum_matching(&odd_subgraph);

    for i in max_match.edges() {
        println!("{:?} {:?}", i.0, i.1);

        mst.add_edge(
            i.0,
            i.1,
            distances[NodeIndex::index(i.0)][NodeIndex::index(i.1)],
        );
    }

    println!("{:?}", Dot::new(&mst));

    let mut path: Vec<usize> = Vec::new();

    // let mut dfs = Dfs::new(&mst, NodeIndex::new(0));
    // while let Some(visited) = dfs.next(&mst) {
    //     path.push(visited.index());
    // }

    // path.push(0);

    // let mut cost: f64 = 0.0;

    // for i in 0..(path.len() - 1) {
    //     cost += distances[path[i]][path[i + 1]];
    // }

    // cost

    (path, -1.0)
}
