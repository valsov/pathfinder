mod dijkstra;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    id: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge<'a> {
    from: &'a Node,
    to: &'a Node,
    weight: usize,
}

impl<'a> Edge<'a> {
    fn new(from: &'a Node, to: &'a Node, weight: usize) -> Self {
        Edge { from, to, weight }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Graph<'a> {
    nodes: &'a [&'a Node],
    edges: &'a [&'a Edge<'a>],
}

impl<'a> Graph<'a> {
    fn new(nodes: &'a [&'a Node], edges: &'a [&'a Edge]) -> Self {
        Graph { nodes, edges }
    }

    fn get_neighbors(&self, node: &'a Node) -> Vec<(&'a Node, usize)> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node)
            .map(|edge| (edge.to, edge.weight))
            .collect()
    }
}

#[derive(Debug)]
struct Path<'a> {
    node: &'a Node,
    cost: usize,
}

trait PathFinder {
    fn shortest_path<'a>(graph: &'a Graph<'a>, start: &'a Node, goal: &'a Node) -> Vec<Path<'a>>;
}

fn main() {
    let node_a = &Node { id: 0 };
    let node_b = &Node { id: 1 };
    let node_c = &Node { id: 2 };
    let node_d = &Node { id: 3 };

    let nodes = vec![node_a, node_b, node_c, node_d];

    let edge_a_b = &Edge::new(node_a, node_b, 1);
    let edge_b_c = &Edge::new(node_b, node_c, 2);
    let edge_a_c = &Edge::new(node_a, node_c, 4);
    let edge_b_d = &Edge::new(node_b, node_d, 7);
    let edge_c_d = &Edge::new(node_c, node_d, 1);

    let edges = vec![edge_a_b, edge_b_c, edge_a_c, edge_b_d, edge_c_d];

    let graph = Graph::new(&nodes, &edges);

    let start_node = &nodes[0];
    let goal_node = &nodes[3];

    let distances = dijkstra::shortest_path(&graph, start_node, goal_node);

    println!("Shortest distances: {:?}", distances);
}
