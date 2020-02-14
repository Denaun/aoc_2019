use itertools::Itertools;
use num::abs;
use num::traits::Signed;
use num_integer::Integer;
use num_rational::Ratio;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Angle<T: Clone + Integer> {
    pub quadrant: Quadrant,
    pub slope: Ratio<T>,
}

impl<T: Clone + Integer> PartialOrd for Angle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Clone + Integer> Ord for Angle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.quadrant.cmp(&other.quadrant) {
            Ordering::Equal => self.slope.cmp(&other.slope), //.reverse(),
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Integer + Signed> Point<T> {
    pub fn angle_with(&self, other: &Self) -> Angle<T> {
        let x = other.x - self.x;
        let y = other.y - self.y;
        match (x.cmp(&T::zero()), y.cmp(&T::zero())) {
            (Ordering::Greater, Ordering::Greater) | (Ordering::Greater, Ordering::Equal) => {
                Angle {
                    quadrant: Quadrant::BottomRight,
                    slope: Ratio::new(y, x),
                }
            }
            (Ordering::Less, Ordering::Greater) | (Ordering::Equal, Ordering::Greater) => Angle {
                quadrant: Quadrant::BottomLeft,
                slope: -Ratio::new(x, y),
            },
            (Ordering::Less, Ordering::Less) | (Ordering::Less, Ordering::Equal) => Angle {
                quadrant: Quadrant::TopLeft,
                slope: Ratio::new(y, x),
            },
            (Ordering::Greater, Ordering::Less) | (Ordering::Equal, Ordering::Less) => Angle {
                quadrant: Quadrant::TopRight,
                slope: -Ratio::new(x, y),
            },
            (Ordering::Equal, Ordering::Equal) => panic!("Coincident points"),
        }
    }

    pub fn distance_from(&self, other: &Self) -> T {
        let angle = self.angle_with(other);
        (match angle.quadrant {
            Quadrant::BottomRight | Quadrant::TopLeft => abs(other.x - self.x),
            Quadrant::BottomLeft | Quadrant::TopRight => abs(other.y - self.y),
        } / *angle.slope.denom())
    }
}

pub trait AsteroidMap<T> {
    fn read(data: &str) -> Self;
    fn best(&self) -> Option<(&Point<T>, usize)>;
}

impl<T: Copy + TryFrom<usize, Error = impl Debug> + Integer + Signed + Hash> AsteroidMap<T>
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

pub fn vaporization_order<'a>(
    map: &'a [Point<isize>],
    source: &Point<isize>,
) -> Vec<&'a Point<isize>> {
    let steps: Vec<Vec<_>> = map
        .iter()
        .filter(|point| point != &source)
        .sorted_by_key(|point| source.angle_with(point))
        .group_by(|point| source.angle_with(point))
        .into_iter()
        .map(|(_, points)| {
            points
                .sorted_by_key(|point| source.distance_from(point))
                .collect()
        })
        .collect();
    let mut result = Vec::with_capacity(map.len());
    for i in 0..steps.iter().map(|vec| vec.len()).max().unwrap() {
        for vec in &steps {
            if let Some(point) = vec.get(i) {
                result.push(*point);
            }
        }
    }
    result
}

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

    #[test]
    fn example6() {
        let map = AsteroidVec::read(
            "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
        );
        let order = vaporization_order(&map, &Point { x: 8, y: 3 });
        assert_eq!(
            order,
            vec![
                // .#....###24...#..
                // ##...##.13#67..9#
                // ##...#...5.8####.
                // ..#.....X...###..
                // ..#.#.....#....##
                &Point { x: 8, y: 1 },
                &Point { x: 9, y: 0 },
                &Point { x: 9, y: 1 },
                &Point { x: 10, y: 0 },
                &Point { x: 9, y: 2 },
                &Point { x: 11, y: 1 },
                &Point { x: 12, y: 1 },
                &Point { x: 11, y: 2 },
                &Point { x: 15, y: 1 },
                // .#....###.....#..
                // ##...##...#.....#
                // ##...#......1234.
                // ..#.....X...5##..
                // ..#.9.....8....76
                &Point { x: 12, y: 2 },
                &Point { x: 13, y: 2 },
                &Point { x: 14, y: 2 },
                &Point { x: 15, y: 2 },
                &Point { x: 12, y: 3 },
                &Point { x: 16, y: 4 },
                &Point { x: 15, y: 4 },
                &Point { x: 10, y: 4 },
                &Point { x: 4, y: 4 },
                // .8....###.....#..
                // 56...9#...#.....#
                // 34...7...........
                // ..2.....X....##..
                // ..1..............
                &Point { x: 2, y: 4 },
                &Point { x: 2, y: 3 },
                &Point { x: 0, y: 2 },
                &Point { x: 1, y: 2 },
                &Point { x: 0, y: 1 },
                &Point { x: 1, y: 1 },
                &Point { x: 5, y: 2 },
                &Point { x: 1, y: 0 },
                &Point { x: 5, y: 1 },
                // ......234.....6..
                // ......1...5.....7
                // .................
                // ........X....89..
                // .................
                &Point { x: 6, y: 1 },
                &Point { x: 6, y: 0 },
                &Point { x: 7, y: 0 },
                &Point { x: 8, y: 0 },
                &Point { x: 10, y: 1 },
                &Point { x: 14, y: 0 },
                &Point { x: 16, y: 1 },
                &Point { x: 13, y: 3 },
                &Point { x: 14, y: 3 },
            ]
        );
    }

    #[test]
    fn example7() {
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
        let order = vaporization_order(&map, map.best().unwrap().0);
        assert_eq!(order[0], &Point { x: 11, y: 12 });
        assert_eq!(order[1], &Point { x: 12, y: 1 });
        assert_eq!(order[2], &Point { x: 12, y: 2 });
        assert_eq!(order[9], &Point { x: 12, y: 8 });
        assert_eq!(order[19], &Point { x: 16, y: 0 });
        assert_eq!(order[49], &Point { x: 16, y: 9 });
        assert_eq!(order[99], &Point { x: 10, y: 16 });
        assert_eq!(order[198], &Point { x: 9, y: 6 });
        assert_eq!(order[199], &Point { x: 8, y: 2 });
        assert_eq!(order[200], &Point { x: 10, y: 9 });
        assert_eq!(order[298], &Point { x: 11, y: 1 });
    }

    #[test]
    fn day_10_part_2() {
        let map = AsteroidVec::read(include_str!("input"));
        let (point, _) = map.best().unwrap();
        let order = vaporization_order(&map, point);
        assert_eq!(order[199].x * 100 + order[199].y, 616);
    }
}
