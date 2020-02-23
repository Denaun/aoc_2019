use day_9::computer::Computer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates(isize, isize);

impl Coordinates {
    pub fn neighbor(self, dir: Direction) -> Self {
        match dir {
            Direction::North => Coordinates(self.0 + 1, self.1),
            Direction::South => Coordinates(self.0 - 1, self.1),
            Direction::West => Coordinates(self.0, self.1 - 1),
            Direction::East => Coordinates(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}
impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        DIRECTIONS.iter()
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exploration {
    InProgress,
    Finished,
}

type Path = Vec<Direction>;

#[derive(Debug)]
struct Target(Coordinates, Path);

pub struct Explorer {
    dir_queue: Path,
    target: Target,
    to_visit: VecDeque<Target>,
    visited: HashSet<Coordinates>,
}

impl Explorer {
    pub fn new() -> Self {
        let base = Coordinates(0, 0);
        let mut to_visit: VecDeque<Target> = Direction::iter()
            .cloned()
            .map(|dir| Target(base.neighbor(dir), vec![dir]))
            .collect();
        let target = to_visit.pop_front().unwrap();
        Explorer {
            dir_queue: target.1.clone(),
            target,
            to_visit,
            visited: [base].iter().cloned().collect(),
        }
    }

    pub fn next_direction(&self) -> &Direction {
        self.dir_queue.last().unwrap()
    }

    pub fn notify_wall(&mut self) -> Exploration {
        assert_eq!(
            self.dir_queue.len(),
            1,
            "Shouldn't run into walls for known paths"
        );
        self.visited.insert(self.target.0);
        if let Some(next_target) = self.to_visit.pop_front() {
            self.dir_queue = Self::find_path(&self.target, &next_target);
            self.dir_queue.pop().unwrap(); // Never actually reached the current target.
            self.target = next_target;
            Exploration::InProgress
        } else {
            Exploration::Finished
        }
    }

    pub fn notify_space(&mut self) -> Exploration {
        self.dir_queue.pop().unwrap();
        if self.dir_queue.is_empty() {
            let pos = &self.target.0;
            let path = &self.target.1;
            self.visited.insert(self.target.0);
            for dir in Direction::iter().cloned() {
                let neighbor = pos.neighbor(dir);
                if !self.visited.contains(&neighbor) {
                    let mut path = path.clone();
                    path.push(dir);
                    self.to_visit.push_back(Target(neighbor, path));
                }
            }
            if let Some(next_target) = self.to_visit.pop_front() {
                self.dir_queue = Self::find_path(&self.target, &next_target);
                self.target = next_target;
                Exploration::InProgress
            } else {
                Exploration::Finished
            }
        } else {
            Exploration::InProgress
        }
    }

    pub fn get_target_path(&self) -> Path {
        self.target.1.clone()
    }

    fn find_path(from: &Target, to: &Target) -> Path {
        let mut from = from.1.iter().peekable();
        let mut to = to.1.iter().peekable();
        // Manual zip + skip_while as zip requires both iterators to have the
        // same length. Also this avoids unzip, which would two vectors.
        while from.peek() == to.peek() {
            from.next();
            to.next();
        }
        from.cloned()
            .map(Direction::opposite)
            .rev()
            .chain(to.cloned())
            .rev() // The path will be applied by popping from the end.
            .collect()
    }
}

impl Default for Explorer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn find_oxygen_system(intcode: Vec<isize>) -> Path {
    struct State {
        explorer: Explorer,
        path: Option<Path>,
    }
    let state = RefCell::new(State {
        explorer: Explorer::new(),
        path: None,
    });
    let mut computer = Computer::new(
        intcode,
        || *state.borrow().explorer.next_direction() as isize,
        |v| {
            let mut state = state.borrow_mut();
            match v {
                0 => {
                    let e = state.explorer.notify_wall();
                    assert_eq!(e, Exploration::InProgress)
                }
                1 => {
                    let e = state.explorer.notify_space();
                    assert_eq!(e, Exploration::InProgress)
                }
                2 => state.path = Some(state.explorer.get_target_path()),
                _ => panic!(),
            }
        },
    );
    while state.borrow().path.is_none() {
        let ok = computer.run_one().unwrap();
        assert!(ok);
    }
    state.into_inner().path.unwrap()
}

type AdjList<T> = HashMap<T, HashSet<T>>;

trait AdjInsert<T> {
    fn adj_insert(&mut self, x: T, y: T);
}
impl<T> AdjInsert<T> for AdjList<T>
where
    T: Clone + Eq + Hash,
{
    fn adj_insert(&mut self, x: T, y: T) {
        self.entry(x.clone()).or_default().insert(y.clone());
        self.entry(y).or_default().insert(x);
    }
}

pub fn build_map(intcode: Vec<isize>) -> (Coordinates, AdjList<Coordinates>) {
    struct State {
        pos: Coordinates,
        dir: Option<Direction>,
        stop: bool,
        center: Option<Coordinates>,
        map: AdjList<Coordinates>,
        explorer: Explorer,
    }
    let state = RefCell::new(State {
        pos: Coordinates(0, 0),
        dir: None,
        stop: false,
        center: None,
        map: AdjList::new(),
        explorer: Explorer::new(),
    });
    let mut computer = Computer::new(
        intcode,
        || {
            let mut state = state.borrow_mut();
            let dir = *state.explorer.next_direction();
            state.dir = Some(dir);
            dir as isize
        },
        |v| {
            let mut state = state.borrow_mut();
            state.stop = match v {
                0 => state.explorer.notify_wall(),
                1 => {
                    let new_pos = state.pos.neighbor(state.dir.unwrap());
                    let old_pos = state.pos;
                    state.map.adj_insert(old_pos, new_pos);
                    state.pos = new_pos;
                    state.explorer.notify_space()
                }
                2 => {
                    let new_pos = state.pos.neighbor(state.dir.unwrap());
                    let old_pos = state.pos;
                    state.map.adj_insert(old_pos, new_pos);
                    state.pos = new_pos;
                    match state.center {
                        None => state.center = Some(new_pos),
                        Some(pos) if pos == new_pos => (),
                        _ => panic!("More than one center"),
                    }
                    state.explorer.notify_space()
                }
                _ => panic!(),
            } == Exploration::Finished;
        },
    );
    while !state.borrow().stop {
        let ok = computer.run_one().unwrap();
        assert!(ok);
    }
    let state = state.into_inner();
    (state.center.unwrap(), state.map)
}

pub fn longest_distance<T>(center: T, map: &AdjList<T>) -> usize
where
    T: Eq + Hash,
{
    let mut visited: HashSet<&T> = HashSet::new();
    let mut to_visit: HashSet<&T> = [&center].iter().cloned().collect();
    for turn in 0.. {
        visited.extend(to_visit.iter());
        to_visit = map
            .iter()
            .filter_map(|(c, adj)| {
                if to_visit.contains(c) {
                    Some(adj)
                } else {
                    None
                }
            })
            .flatten()
            .filter(|c| !visited.contains(c))
            .collect();
        if to_visit.is_empty() {
            return turn;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_intcode(data: &str) -> Vec<isize> {
        data.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn day_15_part_1() {
        let path = find_oxygen_system(read_intcode(include_str!("input")));
        assert_eq!(path.len(), 270);
    }

    #[test]
    fn empty_map() {
        assert_eq!(longest_distance(0, &AdjList::new()), 0);
    }

    #[test]
    fn single() {
        let mut adj = AdjList::new();
        adj.adj_insert(0, 1);
        assert_eq!(longest_distance(0, &adj), 1);
    }

    #[test]
    fn fork() {
        let mut adj = AdjList::new();
        adj.adj_insert(0, 1);
        adj.adj_insert(1, 2);
        adj.adj_insert(1, 3);
        assert_eq!(longest_distance(0, &adj), 2);
    }

    #[test]
    fn cycle() {
        let mut adj = AdjList::new();
        adj.adj_insert(0, 1);
        adj.adj_insert(1, 2);
        adj.adj_insert(2, 0);
        assert_eq!(longest_distance(0, &adj), 1);
    }

    #[test]
    fn day_15_part_2() {
        let (center, map) = build_map(read_intcode(include_str!("input")));
        assert_eq!(center, Coordinates(-18, -20));
        assert_eq!(longest_distance(center, &map), 364);
    }
}
