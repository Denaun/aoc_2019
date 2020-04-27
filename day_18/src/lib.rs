mod graph;

use graph::{Coordinates, GraphModel, Node, UniqueNode};
use std::collections::HashSet;

impl GraphModel for Vec<Vec<char>> {
    fn neighbors(&self, position: &Coordinates) -> Vec<(Coordinates, Node, usize)> {
        let (x0, y0) = position;
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(dx, dy)| ((*x0 as isize + dx) as usize, (*y0 as isize + dy) as usize))
            .filter_map(|coord| {
                let (x, y) = coord;
                self.get(y)
                    .and_then(|line| line.get(x))
                    .map(|c| match c {
                        '@' => Node::Unique(UniqueNode::Root),
                        '.' => Node::Empty,
                        '#' => Node::Wall,
                        c @ 'a'..='z' => Node::Unique(UniqueNode::Key(*c)),
                        c @ 'A'..='Z' => Node::Unique(UniqueNode::Door(c.to_ascii_lowercase())),
                        _ => unreachable!(),
                    })
                    .map(|node| (coord, node, 1))
            })
            .collect()
    }

    fn find(&self, node: UniqueNode) -> Option<(usize, usize)> {
        let c = match node {
            UniqueNode::Root => '@',
            UniqueNode::Key(c) => c,
            UniqueNode::Door(c) => c.to_ascii_uppercase(),
        };
        self.iter().enumerate().find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, &c1)| if c1 == c { Some((x, y)) } else { None })
        })
    }

    fn keys(&self) -> HashSet<char> {
        self.iter()
            .flat_map(|line| {
                line.iter().filter(|c| match c {
                    'a'..='z' => true,
                    _ => false,
                })
            })
            .cloned()
            .collect()
    }
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
            Graph::new(&str_to_mat(
                "#########\n\
                 #b.A.@.a#\n\
                 #########"
            ))
            .shortest_path_length(),
            8
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Graph::new(&str_to_mat(
                "########################\n\
                 #f.D.E.e.C.b.A.@.a.B.c.#\n\
                 ######################.#\n\
                 #d.....................#\n\
                 ########################"
            ))
            .shortest_path_length(),
            86
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Graph::new(&str_to_mat(
                "########################\n\
                 #...............b.C.D.f#\n\
                 #.######################\n\
                 #.....@.a.B.c.d.A.e.F.g#\n\
                 ########################"
            ))
            .shortest_path_length(),
            132
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Graph::new(&str_to_mat(
                "#################\n\
                 #i.G..c...e..H.p#\n\
                 ########.########\n\
                 #j.A..b...f..D.o#\n\
                 ########@########\n\
                 #k.E..a...g..B.n#\n\
                 ########.########\n\
                 #l.F..d...h..C.m#\n\
                 #################"
            ))
            .shortest_path_length(),
            136
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Graph::new(&str_to_mat(
                "########################\n\
                 #@..............ac.GI.b#\n\
                 ###d#e#f################\n\
                 ###A#B#C################\n\
                 ###g#h#i################\n\
                 ########################"
            ))
            .shortest_path_length(),
            81
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            Graph::new(&str_to_mat(include_str!("input"))).shortest_path_length(),
            136
        );
    }
}
