use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    node: &'a Node,
    cost: usize,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<'a>(graph: &'a Graph<'a>, start: &'a Node, goal: &'a Node) -> Vec<Path<'a>> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::<&Node, usize>::new();

    heap.push(State {
        node: start,
        cost: 0,
    });

    while let Some(State { node, cost }) = heap.pop() {
        if visited.contains(node) {
            continue;
        }

        if node == goal {
            return to_ordered_path(distances);
        }

        visited.insert(node);

        for (neighbor, weight) in graph.get_neighbors(node) {
            let next_cost = cost + weight;
            if !distances.contains_key(neighbor) || next_cost < *distances.get(neighbor).unwrap() {
                distances.insert(neighbor, next_cost);
                heap.push(State {
                    node: neighbor,
                    cost: next_cost,
                });
            }
        }
    }

    to_ordered_path(distances)
}

fn to_ordered_path(map: HashMap<&Node, usize>) -> Vec<Path<'_>> {
    let mut vec = map
        .iter()
        .map(|d| Path {
            node: d.0,
            cost: *d.1,
        })
        .collect::<Vec<Path<'_>>>();
    vec.sort_by_key(|p| p.cost);
    vec
}
