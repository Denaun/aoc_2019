use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UniqueNode {
    Root,
    Key(char),
    Door(char),
}
pub enum Node {
    Unique(UniqueNode),
    Empty,
    Wall,
}

pub type Coordinates = (usize, usize);
pub type Cost = usize;

pub trait GraphModel {
    fn neighbors(&self, position: &Coordinates) -> Vec<(Coordinates, Node, Cost)>;
    fn find(&self, node: UniqueNode) -> Option<Coordinates>;
    fn keys(&self) -> HashSet<char>;

    fn find_root(&self) -> Coordinates {
        self.find(UniqueNode::Root).unwrap()
    }
}

pub struct Graph<'a, T>
where
    T: GraphModel,
{
    model: &'a T,
    position: Coordinates,
    keys: HashSet<char>,
    visited: HashMap<Coordinates, HashSet<char>>,
}

impl<'a, T> Graph<'a, T>
where
    T: GraphModel,
{
    pub fn new(model: &'a T) -> Self {
        Graph {
            model,
            position: model.find_root(),
            keys: HashSet::new(),
            visited: HashMap::new(),
        }
    }

    pub fn shortest_path_length(&self) -> usize {
        for steps in 0.. {
            if self.has_solution(steps) {
                return steps;
            }
        }
        unreachable!();
    }

    fn has_solution(&self, steps: usize) -> bool {
        if steps == 0 {
            return self.keys == self.model.keys();
        }
        self.model
            .neighbors(&self.position)
            .into_iter()
            .any(|(position, node, cost)| {
                if steps < cost {
                    return false;
                }
                if self
                    .visited
                    .get(&position)
                    .map(|ks| self.keys.is_subset(&ks))
                    .unwrap_or(false)
                {
                    return false;
                }
                let mut visited = self.visited.clone();
                visited.entry(position).or_default().extend(&self.keys);
                match node {
                    Node::Wall => false,
                    Node::Unique(UniqueNode::Door(c)) if !self.keys.contains(&c) => false,
                    Node::Unique(UniqueNode::Key(c)) => {
                        let mut keys = self.keys.clone();
                        keys.insert(c);
                        Graph {
                            model: self.model,
                            position,
                            keys,
                            visited,
                        }
                        .has_solution(steps - cost)
                    }
                    _ => Graph {
                        model: self.model,
                        position,
                        keys: self.keys.clone(),
                        visited,
                    }
                    .has_solution(steps - cost),
                }
            })
    }
}
