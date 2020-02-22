use day_9::computer::Computer;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates(isize, isize);

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

    pub fn reached_wall(&mut self) {
        assert_eq!(
            self.dir_queue.len(),
            1,
            "Shouldn't run into walls for known paths"
        );
        self.visited.insert(self.target.0);
        let next_target = self.to_visit.pop_front().unwrap();
        self.dir_queue = Self::find_path(&self.target, &next_target);
        self.dir_queue.pop().unwrap(); // Never actually reached the current target.
        self.target = next_target;
    }

    pub fn reached_space(&mut self) {
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
            let next_target = self.to_visit.pop_front().unwrap();
            self.dir_queue = Self::find_path(&self.target, &next_target);
            self.target = next_target;
        }
    }

    pub fn reached_oxygen_system(&mut self) -> Path {
        self.dir_queue.pop().unwrap();
        assert!(self.dir_queue.is_empty(), "Already visited");
        self.target.1.clone()
    }

    fn find_path(from: &Target, to: &Target) -> Path {
        from.1
            .iter()
            .cloned()
            .map(Direction::opposite)
            .rev()
            .chain(to.1.iter().cloned())
            .rev() // The path will be applied by popping from the end.
            .collect()
    }
}

impl Default for Explorer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_explorer(intcode: Vec<isize>) -> Path {
    let path = RefCell::new(None);
    let explorer = RefCell::new(Explorer::new());
    let mut computer = Computer::new(
        intcode,
        || *explorer.borrow().next_direction() as isize,
        |v| match v {
            0 => explorer.borrow_mut().reached_wall(),
            1 => explorer.borrow_mut().reached_space(),
            2 => *path.borrow_mut() = Some(explorer.borrow_mut().reached_oxygen_system()),
            _ => panic!(),
        },
    );
    while path.borrow().is_none() {
        let ok = computer.run_one().unwrap();
        assert!(ok);
    }
    path.into_inner().unwrap()
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
        let path = run_explorer(read_intcode(include_str!("input")));
        assert_eq!(path.len(), 270);
    }
}
