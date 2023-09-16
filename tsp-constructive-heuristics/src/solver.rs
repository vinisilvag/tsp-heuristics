use petgraph::Graph;

use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};

pub fn christofides(graph: Graph<usize, f32, petgraph::Undirected>) {
    let mut mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&graph));

    println!("{:?}", Dot::new(&mst));

    let mut odd_degree: Vec<NodeIndex> = Vec::new();

    for i in mst.node_indices() {
        let neighbors = mst.neighbors(i);
        let node_degree = neighbors.count();

        if node_degree % 2 != 0 {
            odd_degree.push(i);
        }
    }

    // min cost perfect match M

    // add all edges from M to T forming a new graph C

    // find a Eulerian cycle of C

    // remove all repeated vertices of C

    println!("{:?}", odd_degree);
}
