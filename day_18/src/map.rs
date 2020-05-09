use crate::graph::GraphNode;
use crate::Coordinates;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapNode {
    Filled(GraphNode),
    Empty,
}

pub trait Map {
    fn node_at(&self, position: &Coordinates) -> Option<MapNode>;
    fn neighbors(&self, position: &Coordinates) -> Vec<Coordinates>;
    fn find(&self, node: GraphNode) -> Option<Coordinates>;

    fn find_root(&self, index: Option<u8>) -> Option<Coordinates> {
        self.find(GraphNode::Root(index))
    }
}

impl Map for Vec<Vec<char>> {
    fn node_at(&self, position: &Coordinates) -> Option<MapNode> {
        let &(x, y) = position;
        self.get(y)
            .and_then(|line| line.get(x))
            .and_then(|c| match c {
                '@' => Some(MapNode::Filled(GraphNode::Root(None))),
                '.' => Some(MapNode::Empty),
                '#' => None,
                c @ '0'..='9' => Some(MapNode::Filled(GraphNode::Root(Some(
                    c.to_digit(10).unwrap() as u8,
                )))),
                c @ 'a'..='z' => Some(MapNode::Filled(GraphNode::Key(*c))),
                c @ 'A'..='Z' => Some(MapNode::Filled(GraphNode::Door(c.to_ascii_lowercase()))),
                _ => unreachable!(),
            })
    }
    fn neighbors(&self, position: &Coordinates) -> Vec<Coordinates> {
        let (x0, y0) = position;
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(dx, dy)| ((*x0 as isize + dx) as usize, (*y0 as isize + dy) as usize))
            .filter(|coord| self.node_at(&coord).is_some())
            .collect()
    }

    fn find(&self, node: GraphNode) -> Option<Coordinates> {
        let c = match node {
            GraphNode::Root(c) => c.map(|c| (c + '0' as u8) as char).unwrap_or('@'),
            GraphNode::Key(c) => c,
            GraphNode::Door(c) => c.to_ascii_uppercase(),
        };
        self.iter().enumerate().find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, &c1)| if c1 == c { Some((x, y)) } else { None })
        })
    }
}
