fn main() {
    // Create graph

    // Compute best path
}

struct Graph {
    nodes: Vec<Node>,
    paths: Vec<Path>,
}

struct Node {
    name: &'static str,
}

struct Path {
    from: &'static str,
    to: &'static str,
    weight: usize,
}
