pub mod alignment;

use day_9::computer::Computer;
use std::char;

pub fn get_view(intcode: Vec<isize>) -> String {
    let mut data = String::new();
    Computer::new(
        intcode,
        || unreachable!(),
        |v| {
            data.push(char::from_u32(v as u32).unwrap());
        },
    )
    .run()
    .unwrap();
    data
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub turn: Turn,
    pub distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, dir: Turn) -> Self {
        match dir {
            Turn::Left => match self {
                Self::North => Self::West,
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
            },
            Turn::Right => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbour(self, dir: Direction) -> Option<Self> {
        self.mv(dir, 1)
    }

    fn mv(self, dir: Direction, distance: usize) -> Option<Self> {
        match dir {
            Direction::North => {
                if self.y >= distance {
                    Some(Self {
                        x: self.x,
                        y: self.y - distance,
                    })
                } else {
                    None
                }
            }
            Direction::East => {
                if self.x <= usize::max_value() - distance {
                    Some(Self {
                        x: self.x + distance,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::South => {
                if self.y <= usize::max_value() - distance {
                    Some(Self {
                        x: self.x,
                        y: self.y + distance,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if self.x >= distance {
                    Some(Self {
                        x: self.x - distance,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }
}

trait Grid<T> {
    fn cell(&self, coord: Coord) -> Option<&T>;
}

impl Grid<u8> for [&str] {
    fn cell(&self, coord: Coord) -> Option<&u8> {
        self.get(coord.y)
            .and_then(|line| line.as_bytes().get(coord.x))
    }
}

pub fn find_path(data: &[&str]) -> Vec<Move> {
    let mut dir = Direction::North;
    let mut pos = data
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            if let Some(x) =
                line.chars()
                    .enumerate()
                    .find_map(|(x, ch)| if ch == '^' { Some(x) } else { None })
            {
                Some(Coord { x, y })
            } else {
                None
            }
        })
        .unwrap();
    let mut moves = Vec::new();
    while let Some(next) = [Turn::Left, Turn::Right]
        .iter()
        .copied()
        .filter_map(|turn| {
            let dir = dir.turn(turn);
            let mut pos = pos.neighbour(dir);
            let mut distance = 0;
            while pos.and_then(|pos| data.cell(pos)) == Some(&b'#') {
                distance += 1;
                pos = pos.unwrap().neighbour(dir);
            }
            if distance > 0 {
                Some(Move { turn, distance })
            } else {
                None
            }
        })
        .next()
    {
        dir = dir.turn(next.turn);
        pos = pos.mv(dir, next.distance).unwrap();
        moves.push(next);
    }
    moves
}

/// Find the longest sequence from the start of the first slice with a repetition
/// either in the same slice without intersections, or in the remaining slices.
/// Returns the ending index of the sequence.
fn find_repeated_sequence<T>(slices: &[&[T]]) -> usize
where
    T: PartialEq,
{
    let data = slices[0];
    let first = (1..=data.len() / 2)
        .rev()
        .find(|len| {
            let base = &data[0..*len];
            data.windows(*len).skip(*len).any(|seq| base == seq)
        })
        .unwrap_or(0);
    slices[1..]
        .iter()
        .filter_map(|other| {
            let max_len = std::cmp::min(data.len(), other.len());
            (1..=max_len).rev().find(|len| {
                let base = &data[0..*len];
                other.windows(*len).any(|seq| base == seq)
            })
        })
        .fold(first, std::cmp::max)
}

fn encode(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| {
            format!(
                "{},{}",
                match m.turn {
                    Turn::Left => 'L',
                    Turn::Right => 'R',
                },
                m.distance
            )
        })
        .collect::<Vec<String>>()
        .join(",")
}

pub fn clean_scaffolding_input(view: &[&str]) -> String {
    let path = find_path(view);
    // Ad-hoc algorithm to split into repeated sequences. Unlikely to find the optimal solution.
    let mut paths = vec![path.as_slice()];
    let mut routines = Vec::new();
    while !paths.is_empty() {
        assert!(routines.len() < 3);
        let routine = &paths[0][..find_repeated_sequence(&paths)];
        // Find all the occurrences of the routine and remove them.
        paths = paths
            .into_iter()
            .flat_map(|mut slice| {
                let mut new_paths = Vec::new();
                while let Some(start) = slice
                    .windows(routine.len())
                    .enumerate()
                    .find(|(_, seq)| routine == *seq)
                    .map(|(i, _)| i)
                {
                    if start > 0 {
                        new_paths.push(&slice[..start]);
                    }
                    slice = &slice[start + routine.len()..];
                }
                if !slice.is_empty() {
                    new_paths.push(slice);
                }
                new_paths
            })
            .collect();
        // Store the new routine.
        let name = char::from_u32('A' as u32 + routines.len() as u32)
            .unwrap()
            .to_string();
        routines.push((name, routine.to_vec()));
    }
    // Determine the call order.
    let mut calls = Vec::new();
    let mut rest = path.as_slice();
    while !rest.is_empty() {
        let (name, routine) = routines
            .iter()
            .find(|(_, routine)| rest.starts_with(&routine))
            .unwrap();
        calls.push(name.clone());
        rest = &rest[routine.len()..];
    }
    // Build the input.
    [
        calls.join(&","),
        routines
            .iter()
            .map(|(_, routine)| encode(&routine))
            .collect::<Vec<_>>()
            .join("\n"),
        "n\n".to_owned(),
    ]
    .join("\n")
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
    fn day_17_part_1() {
        let view = get_view(read_intcode(include_str!("input")));
        let data: Vec<_> = view.lines().collect();
        assert_eq!(alignment::alignment_parameter(&data), 5620);
    }

    #[test]
    fn example_2() {
        let view: Vec<_> = "#######...#####\n\
                            #.....#...#...#\n\
                            #.....#...#...#\n\
                            ......#...#...#\n\
                            ......#...###.#\n\
                            ......#.....#.#\n\
                            ^########...#.#\n\
                            ......#.#...#.#\n\
                            ......#########\n\
                            ........#...#..\n\
                            ....#########..\n\
                            ....#...#......\n\
                            ....#...#......\n\
                            ....#...#......\n\
                            ....#####......"
            .lines()
            .collect();
        let path = find_path(&view);
        assert_eq!(
            encode(&path),
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        );
        let input = clean_scaffolding_input(&view);
        // NOTE: The reference puts "R,8" at the end of B instead of the start
        // of C. The two are equivalent.
        assert_eq!(
            input,
            "A,B,C,B,A,C\n\
             R,8,R,8\n\
             R,4,R,4\n\
             R,8,L,6,L,2\n\
             n\n"
        )
    }

    #[test]
    fn day_17_part_2() {
        let mut intcode = read_intcode(include_str!("input"));
        assert_eq!(intcode[0], 1);
        let view = get_view(intcode.clone());
        let view: Vec<_> = view.lines().collect();
        let mut input: Vec<_> = clean_scaffolding_input(&view).chars().rev().collect();
        let mut dust = None;
        intcode[0] = 2;
        Computer::new(
            intcode,
            || input.pop().unwrap() as isize,
            |v| dust = Some(v),
        )
        .run()
        .unwrap();
        assert_eq!(dust, Some(768_115));
    }
}
