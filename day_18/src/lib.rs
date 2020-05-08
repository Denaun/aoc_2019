pub mod graph;
pub mod map;

use graph::{Graph, GraphNode};
use std::collections::{BTreeSet, BinaryHeap, HashMap};

pub type Coordinates = (usize, usize);
pub type Cost = usize;
pub type KeyId = char;

#[derive(Clone, PartialEq, Eq)]
struct State {
    node: GraphNode,
    cost: Cost,
    keys: BTreeSet<KeyId>,
    path: Vec<GraphNode>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.path.len().cmp(&other.path.len()))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path_length(graph: &Graph) -> usize {
    let all_keys = graph.keys();
    let mut visited = HashMap::<GraphNode, HashMap<BTreeSet<KeyId>, Cost>>::new();
    let mut to_visit: BinaryHeap<_> = [State {
        node: GraphNode::Root,
        cost: 0,
        keys: BTreeSet::new(),
        path: Vec::new(),
    }]
    .iter()
    .cloned()
    .collect();
    while let Some(State {
        node,
        cost,
        keys,
        path,
    }) = to_visit.pop()
    {
        if keys == all_keys {
            return cost;
        }
        if visited
            .get(&node)
            .and_then(|c| c.get(&keys))
            .filter(|&&c| c <= cost)
            .is_some()
        {
            continue;
        }
        for (node, step_cost) in graph.neighbors(&node, &keys) {
            let cost = cost + step_cost;
            let mut keys = keys.clone();
            if let GraphNode::Key(k) = node {
                keys.insert(k);
            }
            let mut path = path.clone();
            path.push(node);
            to_visit.push(State {
                node,
                cost,
                keys,
                path,
            });
        }
        visited.entry(node).or_default().insert(keys, cost);
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    fn str_to_mat(data: &str) -> Vec<Vec<char>> {
        data.lines().map(|line| line.chars().collect()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(
                "#########\n\
                 #b.A.@.a#\n\
                 #########",
            ))),
            8
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(
                "########################\n\
                 #f.D.E.e.C.b.A.@.a.B.c.#\n\
                 ######################.#\n\
                 #d.....................#\n\
                 ########################"
            ))),
            86
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(
                "########################\n\
                 #...............b.C.D.f#\n\
                 #.######################\n\
                 #.....@.a.B.c.d.A.e.F.g#\n\
                 ########################"
            ))),
            132
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(
                "#################\n\
                 #i.G..c...e..H.p#\n\
                 ########.########\n\
                 #j.A..b...f..D.o#\n\
                 ########@########\n\
                 #k.E..a...g..B.n#\n\
                 ########.########\n\
                 #l.F..d...h..C.m#\n\
                 #################",
            ))),
            136
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(
                "########################\n\
                 #@..............ac.GI.b#\n\
                 ###d#e#f################\n\
                 ###A#B#C################\n\
                 ###g#h#i################\n\
                 ########################"
            ))),
            81
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            shortest_path_length(&Graph::new(&str_to_mat(include_str!("input")))),
            2796
        );
    }
}
