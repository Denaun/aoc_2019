use super::map::{Map, MapNode};
use super::{Coordinates, Cost, KeyId};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphNode {
    Root,
    Key(KeyId),
    Door(KeyId),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Graph {
    adj_list: HashMap<GraphNode, HashMap<GraphNode, Cost>>,
}

fn graph_neighbors(map: &impl Map, position: &Coordinates) -> Vec<(Coordinates, Cost)> {
    let mut result = Vec::new();
    let mut visited = [*position].iter().copied().collect::<HashSet<_>>();
    let mut to_visit = [(*position, 0)].iter().copied().collect::<VecDeque<_>>();
    while let Some((current, cost)) = to_visit.pop_front() {
        for position in map.neighbors(&current) {
            if visited.contains(&position) {
                continue;
            }
            let cost = cost + 1;
            match map.node_at(&position) {
                Some(MapNode::Filled(_)) => result.push((position, cost)),
                Some(MapNode::Empty) => to_visit.push_back((position, cost)),
                None => (),
            }
            visited.insert(position);
        }
    }
    result
}

impl Graph {
    pub fn new(map: &impl Map) -> Self {
        let mut adj_list = HashMap::<GraphNode, HashMap<GraphNode, usize>>::new();
        let mut visited = HashSet::new();
        let mut to_visit = [(map.find_root(), GraphNode::Root)]
            .iter()
            .copied()
            .collect::<VecDeque<_>>();
        while let Some((position, node)) = to_visit.pop_front() {
            for (position, cost) in graph_neighbors(map, &position) {
                let neighbor = match map.node_at(&position).unwrap() {
                    MapNode::Filled(n) => n,
                    _ => unreachable!(),
                };
                adj_list.entry(node).or_default().insert(neighbor, cost);
                adj_list.entry(neighbor).or_default().insert(node, cost);
                if !visited.contains(&position) {
                    to_visit.push_back((position, neighbor));
                }
            }
            visited.insert(position);
        }
        Self { adj_list }
    }

    pub fn neighbors(&self, node: &GraphNode, keys: &BTreeSet<KeyId>) -> Vec<(GraphNode, Cost)> {
        let mut result = Vec::new();
        let mut visited = [*node].iter().copied().collect::<HashSet<_>>();
        let mut to_visit = [(*node, 0)].iter().copied().collect::<VecDeque<_>>();
        while let Some((current, cost)) = to_visit.pop_front() {
            for (&node, step_cost) in self.adj_list.get(&current).unwrap() {
                if visited.contains(&node) {
                    continue;
                }
                let cost = cost + step_cost;
                match &node {
                    GraphNode::Key(k) if !keys.contains(k) => result.push((node, cost)),
                    GraphNode::Door(k) if !keys.contains(k) => (),
                    _ => to_visit.push_back((node, cost)),
                }
                visited.insert(node);
            }
        }
        result
    }

    pub fn keys(&self) -> BTreeSet<KeyId> {
        self.adj_list
            .iter()
            .filter_map(|(node, _)| match node {
                GraphNode::Key(k) => Some(*k),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_mat(data: &str) -> Vec<Vec<char>> {
        data.lines().map(|line| line.chars().collect()).collect()
    }

    #[test]
    fn example_1() {
        let map = str_to_mat(
            "#########\n\
             #b.A.@.a#\n\
             #########",
        );
        let graph = Graph::new(&map);
        assert_eq!(
            graph.adj_list,
            [
                (
                    GraphNode::Root,
                    [(GraphNode::Door('a'), 2), (GraphNode::Key('a'), 2)]
                        .iter()
                        .copied()
                        .collect(),
                ),
                (
                    GraphNode::Key('a'),
                    [(GraphNode::Root, 2)].iter().copied().collect()
                ),
                (
                    GraphNode::Door('a'),
                    [(GraphNode::Key('b'), 2), (GraphNode::Root, 2)]
                        .iter()
                        .copied()
                        .collect()
                ),
                (
                    GraphNode::Key('b'),
                    [(GraphNode::Door('a'), 2)].iter().copied().collect()
                )
            ]
            .iter()
            .cloned()
            .collect()
        );
        assert_eq!(graph.keys(), ['a', 'b'].iter().cloned().collect());
    }
}
