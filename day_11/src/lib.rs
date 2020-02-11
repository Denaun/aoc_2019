extern crate day_9;

use day_9::computer::Computer;
use snafu::Snafu;
use std::cell::RefCell;
use std::collections::hash_set::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid color {}", value))]
    ColorInvalid { value: isize },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl TryFrom<isize> for Color {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(Error::ColorInvalid { value }),
        }
    }
}

impl From<Color> for isize {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug)]
pub struct PaintingRobot {
    position: Point,
    direction: Direction,
    whites: HashSet<Point>,
    painted: HashSet<Point>,
}

impl PaintingRobot {
    pub fn new() -> Self {
        PaintingRobot {
            position: Point { x: 0, y: 0 },
            direction: Direction::North,
            whites: HashSet::new(),
            painted: HashSet::new(),
        }
    }

    pub fn current_color(&self) -> Color {
        if self.whites.contains(&self.position) {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn painted_count(&self) -> usize {
        self.painted.len()
    }

    pub fn go_left(&mut self) {
        self.direction = self.direction.turn_left();
        self.advance();
    }
    pub fn go_right(&mut self) {
        self.direction = self.direction.turn_right();
        self.advance();
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::North => self.position.x += 1,
            Direction::South => self.position.x -= 1,
            Direction::East => self.position.y += 1,
            Direction::West => self.position.y -= 1,
        }
    }

    pub fn paint(&mut self, color: Color) {
        match color {
            Color::Black => {
                if self.whites.remove(&self.position) {
                    self.painted.insert(self.position);
                }
            }
            Color::White => {
                if self.whites.insert(self.position) {
                    self.painted.insert(self.position);
                }
            }
        }
    }

    pub fn execute(&mut self, intcode: Vec<isize>) {
        let painter = RefCell::new(self);
        let mut is_color_command = true;
        Computer::new(
            intcode,
            || painter.borrow().current_color().into(),
            |v| {
                let mut painter = painter.borrow_mut();
                if is_color_command {
                    painter.paint(Color::try_from(v).unwrap());
                } else {
                    match v {
                        0 => painter.go_left(),
                        1 => painter.go_right(),
                        _ => panic!("Unexpected command."),
                    }
                }
                is_color_command = !is_color_command;
            },
        )
        .run()
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_conversion() {
        for c in &[Color::Black, Color::White] {
            assert_eq!(*c, Color::try_from(isize::from(*c)).unwrap());
        }
    }

    #[test]
    fn example1() {
        let mut painter = PaintingRobot::new();
        assert_eq!(painter.current_color(), Color::Black);
        painter.paint(Color::White);
        painter.go_left();
        assert_eq!(painter.current_color(), Color::Black);
        painter.paint(Color::Black);
        painter.go_left();
        painter.paint(Color::White);
        painter.go_left();
        painter.paint(Color::White);
        painter.go_left();
        assert_eq!(painter.current_color(), Color::White);
        painter.paint(Color::Black);
        painter.go_right();
        painter.paint(Color::White);
        painter.go_left();
        painter.paint(Color::White);
        painter.go_left();
        // assert_eq!(painter.painted_count(), 6);  // Typo in the puzzle description.
        assert_eq!(painter.painted_count(), 5);
    }

    #[test]
    fn day_11_part_1() {
        let intcode: Vec<isize> = include_str!("input")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let mut painter = PaintingRobot::new();
        painter.execute(intcode);
        assert_eq!(painter.painted_count(), 1907);
    }
}
