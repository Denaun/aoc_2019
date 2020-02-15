use euclid::default::Point3D;
use euclid::default::Vector3D;
use num::Integer;
use num::Signed;
use std::cmp::Ordering;
use std::iter::Sum;

fn component_cmp<T>(lhs: &Point3D<T>, rhs: &Point3D<T>) -> Point3D<Ordering>
where
    T: Ord,
{
    Point3D::new(lhs.x.cmp(&rhs.x), lhs.y.cmp(&rhs.y), lhs.z.cmp(&rhs.z))
}

trait Map<T, U> {
    type Output;

    fn map<F: Fn(T) -> U>(self, f: F) -> Self::Output;
}

impl<T, U> Map<T, U> for Point3D<T> {
    type Output = Point3D<U>;

    fn map<F: Fn(T) -> U>(self, f: F) -> Self::Output {
        Point3D::new(f(self.x), f(self.y), f(self.z))
    }
}

trait Energy<T> {
    fn energy(&self) -> T;
}

impl<T> Energy<T> for Point3D<T>
where
    T: Integer + Signed + Copy + Sum,
{
    fn energy(&self) -> T {
        self.to_array().iter().copied().map(|v| v.abs()).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moon<T> {
    pub position: Point3D<T>,
    pub velocity: Vector3D<T>,
}

impl<T> Moon<T>
where
    T: Integer + Signed + Copy + Sum,
{
    pub fn total_energy(&self) -> T {
        self.potential_energy() * self.kinetic_energy()
    }

    pub fn potential_energy(&self) -> T {
        self.position.energy()
    }

    pub fn kinetic_energy(&self) -> T {
        self.velocity.to_point().energy()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulator<T> {
    pub moons: Vec<Moon<T>>,
}

impl<T> Simulator<T>
where
    T: Integer + Signed + Copy + Sum,
{
    pub fn step(self) -> Self {
        self.gravity_step().velocity_step()
    }

    fn gravity_step(&self) -> Self {
        Self {
            moons: self
                .moons
                .iter()
                .map(|moon| Moon {
                    position: moon.position,
                    velocity: moon.velocity
                        + self.moons.iter().fold(Vector3D::zero(), |sum, other| {
                            sum + component_cmp(&moon.position, &other.position)
                                .map(|o| match o {
                                    Ordering::Equal => T::zero(),
                                    Ordering::Greater => -T::one(),
                                    Ordering::Less => T::one(),
                                })
                                .to_vector()
                        }),
                })
                .collect(),
        }
    }

    fn velocity_step(self) -> Self {
        Self {
            moons: self
                .moons
                .into_iter()
                .map(|moon| Moon {
                    position: moon.position + moon.velocity,
                    velocity: moon.velocity,
                })
                .collect(),
        }
    }

    pub fn energy(&self) -> T {
        self.moons
            .iter()
            .fold(T::zero(), |sum, moon| sum + moon.total_energy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn example1() {
        let mut sim = Simulator {
            moons: vec![
                Moon {
                    position: Point3D::new(-1, 0, 2),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(2, -10, -7),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(4, -8, 8),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(3, 5, -1),
                    velocity: Vector3D::zero(),
                },
            ],
        };
        // Step 1
        sim = sim.step();
        assert_eq!(
            sim,
            Simulator {
                moons: vec![
                    Moon {
                        position: Point3D::new(2, -1, 1),
                        velocity: Vector3D::new(3, -1, -1),
                    },
                    Moon {
                        position: Point3D::new(3, -7, -4),
                        velocity: Vector3D::new(1, 3, 3),
                    },
                    Moon {
                        position: Point3D::new(1, -7, 5),
                        velocity: Vector3D::new(-3, 1, -3),
                    },
                    Moon {
                        position: Point3D::new(2, 2, 0),
                        velocity: Vector3D::new(-1, -3, 1),
                    },
                ],
            }
        );
        for _ in 0..9 {
            sim = sim.step();
        }
        assert_eq!(sim.energy(), 179);
    }

    #[test]
    fn example2() {
        let mut sim = Simulator {
            moons: vec![
                Moon {
                    position: Point3D::new(-8, -10, 0),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(5, 5, 10),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(2, -7, 3),
                    velocity: Vector3D::zero(),
                },
                Moon {
                    position: Point3D::new(9, -8, -3),
                    velocity: Vector3D::zero(),
                },
            ],
        };
        for _ in 0..100 {
            sim = sim.step();
        }
        assert_eq!(sim.energy(), 1940);
    }

    fn read_input(data: &str) -> Simulator<i32> {
        let re = Regex::new(r"<x=((?:-)?\d+), y=((?:-)?\d+), z=((?:-)?\d+)>").unwrap();
        Simulator {
            moons: data
                .lines()
                .map(|line| {
                    let caps = re.captures(line).unwrap();
                    assert_eq!(caps.len(), 4);
                    Moon {
                        position: Point3D::new(
                            caps[1].parse().unwrap(),
                            caps[2].parse().unwrap(),
                            caps[3].parse().unwrap(),
                        ),
                        velocity: Vector3D::zero(),
                    }
                })
                .collect(),
        }
    }

    #[test]
    fn day_12_part_1() {
        let mut sim = read_input(include_str!("input"));
        for _ in 0..1000 {
            sim = sim.step();
        }
        assert_eq!(sim.energy(), 7202);
    }
}
