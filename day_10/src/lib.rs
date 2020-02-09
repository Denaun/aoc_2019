use core::ops::Neg;
use num_integer::Integer;
use num_rational::Ratio;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Quadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Angle<T: Clone + Integer> {
    pub quadrant: Quadrant,
    pub slope: Ratio<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Integer + Neg<Output = T>> Point<T> {
    pub fn angle_with(&self, other: &Self) -> Angle<T> {
        let x = other.x - self.x;
        let y = other.y - self.y;
        match (x.cmp(&T::zero()), y.cmp(&T::zero())) {
            (Ordering::Greater, Ordering::Greater) | (Ordering::Equal, Ordering::Greater) => {
                Angle {
                    quadrant: Quadrant::BottomRight,
                    slope: Ratio::new(x, y),
                }
            }
            (Ordering::Less, Ordering::Greater) | (Ordering::Less, Ordering::Equal) => Angle {
                quadrant: Quadrant::BottomLeft,
                slope: Ratio::new(y, -x),
            },
            (Ordering::Less, Ordering::Less) | (Ordering::Equal, Ordering::Less) => Angle {
                quadrant: Quadrant::TopLeft,
                slope: Ratio::new(-x, -y),
            },
            (Ordering::Greater, Ordering::Less) | (Ordering::Greater, Ordering::Equal) => Angle {
                quadrant: Quadrant::TopRight,
                slope: Ratio::new(-y, x),
            },
            (Ordering::Equal, Ordering::Equal) => panic!("Coincident points"),
        }
    }
}

pub trait AsteroidMap<T> {
    fn read(data: &str) -> Self;
    fn best(&self) -> Option<(&Point<T>, usize)>;
}

impl<T: Copy + TryFrom<usize, Error = impl Debug> + Integer + Neg<Output = T> + Hash> AsteroidMap<T>
    for Vec<Point<T>>
{
    fn read(data: &str) -> Self {
        data.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point {
                            x: T::try_from(x).unwrap(),
                            y: T::try_from(y).unwrap(),
                        })
                    } else {
                        assert!(c == '.');
                        None
                    }
                })
            })
            .collect()
    }

    fn best(&self) -> Option<(&Point<T>, usize)> {
        self.iter()
            .map(|candidate| {
                let visible = self
                    .iter()
                    .filter_map(|point| {
                        if point == candidate {
                            None
                        } else {
                            Some(candidate.angle_with(point))
                        }
                    })
                    .collect::<HashSet<_>>();
                (candidate, visible.len())
            })
            .max_by_key(|(_, count)| *count)
    }
}

pub type AsteroidVec = Vec<Point<isize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let map = AsteroidVec::read(
            "\
.#..#
.....
#####
....#
...##",
        );
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 8);
        assert_eq!(point, &Point { x: 3, y: 4 });
    }

    #[test]
    fn example2() {
        let map = AsteroidVec::read(
            "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        );
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 33);
        assert_eq!(point, &Point { x: 5, y: 8 });
    }

    #[test]
    fn example3() {
        let map = AsteroidVec::read(
            "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
        );
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 35);
        assert_eq!(point, &Point { x: 1, y: 2 });
    }

    #[test]
    fn example4() {
        let map = AsteroidVec::read(
            "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
        );
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 41);
        assert_eq!(point, &Point { x: 6, y: 3 });
    }

    #[test]
    fn example5() {
        let map = AsteroidVec::read(
            "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 210);
        assert_eq!(point, &Point { x: 11, y: 13 });
    }

    #[test]
    fn day_10_part_1() {
        let map = AsteroidVec::read(include_str!("input"));
        let (point, count) = map.best().unwrap();
        assert_eq!(count, 288);
        assert_eq!(point, &Point { x: 17, y: 22 });
    }
}
