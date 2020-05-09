pub mod graph;
pub mod map;

use graph::{Graph, GraphNode};
use std::collections::{BTreeSet, BinaryHeap, HashMap};

pub type Coordinates = (usize, usize);
pub type Cost = usize;
pub type KeyId = char;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    nodes: Vec<GraphNode>,
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
    let mut visited = HashMap::<Vec<GraphNode>, HashMap<BTreeSet<KeyId>, Cost>>::new();
    let mut to_visit: BinaryHeap<_> = [State {
        nodes: graph.roots().into_iter().map(GraphNode::Root).collect(),
        cost: 0,
        keys: BTreeSet::new(),
        path: Vec::new(),
    }]
    .iter()
    .cloned()
    .collect();
    while let Some(State {
        nodes,
        cost,
        keys,
        path,
    }) = to_visit.pop()
    {
        if keys == all_keys {
            println!("{:?}", path);
            return cost;
        }
        if visited
            .get(&nodes)
            .and_then(|c| c.get(&keys))
            .filter(|&&c| c <= cost)
            .is_some()
        {
            continue;
        }
        for (i, node) in nodes.iter().enumerate() {
            for (neighbor, step_cost) in graph.neighbors(node, &keys) {
                let mut nodes = nodes.clone();
                nodes[i] = neighbor;
                let cost = cost + step_cost;
                let mut keys = keys.clone();
                if let GraphNode::Key(k) = neighbor {
                    keys.insert(k);
                }
                let mut path = path.clone();
                path.push(neighbor);
                to_visit.push(State {
                    nodes,
                    cost,
                    keys,
                    path,
                });
            }
        }
        visited.entry(nodes).or_default().insert(keys, cost);
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

    fn make_part_2(mut data: Vec<Vec<char>>) -> Vec<Vec<char>> {
        use crate::map::Map;
        let (x, y) = data.find_root(None).unwrap();
        data[y - 1][x - 1] = '0';
        data[y - 1][x] = '#';
        data[y - 1][x + 1] = '1';
        data[y][x - 1] = '#';
        data[y][x] = '#';
        data[y][x + 1] = '#';
        data[y + 1][x - 1] = '2';
        data[y + 1][x] = '#';
        data[y + 1][x + 1] = '3';
        data
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

    #[test]
    fn example_6() {
        assert_eq!(
            shortest_path_length(&Graph::new(&make_part_2(str_to_mat(
                "#######\n\
                 #a.#Cd#\n\
                 ##...##\n\
                 ##.@.##\n\
                 ##...##\n\
                 #cB#Ab#\n\
                 #######",
            )))),
            8
        );
    }

    #[test]
    fn example_7() {
        assert_eq!(
            shortest_path_length(&Graph::new(&make_part_2(str_to_mat(
                "###############\n\
                 #d.ABC.#.....a#\n\
                 ######...######\n\
                 ######.@.######\n\
                 ######...######\n\
                 #b.....#.....c#\n\
                 ###############",
            )))),
            24
        );
    }

    #[test]
    fn example_8() {
        assert_eq!(
            shortest_path_length(&Graph::new(&make_part_2(str_to_mat(
                "#############\n\
                 #DcBa.#.GhKl#\n\
                 #.###...#I###\n\
                 #e#d#.@.#j#k#\n\
                 ###C#...###J#\n\
                 #fEbA.#.FgHi#\n\
                 #############",
            )))),
            32
        );
    }

    #[test]
    fn example_9() {
        assert_eq!(
            shortest_path_length(&Graph::new(&make_part_2(str_to_mat(
                "#############\n\
                 #g#f.D#..h#l#\n\
                 #F###e#E###.#\n\
                 #dCba...BcIJ#\n\
                 #####.@.#####\n\
                 #nK.L...G...#\n\
                 #M###N#H###.#\n\
                 #o#m..#i#jk.#\n\
                 #############",
            )))),
            72
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            shortest_path_length(&Graph::new(&make_part_2(str_to_mat(include_str!("input"))))),
            2796
        );
    }
}
