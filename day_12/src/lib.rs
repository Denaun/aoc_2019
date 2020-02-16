use num::Integer;
use num::Signed;
use std::iter::Sum;
use std::ops::AddAssign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSlice<T> {
    pub positions: Vec<T>,
    pub velocities: Vec<T>,
}

impl<T> StateSlice<T>
where
    T: Integer + Signed + AddAssign + Sum + Copy,
{
    pub fn step(&mut self) {
        self.gravity_step();
        self.velocity_step();
    }

    fn gravity_step(&mut self) {
        for (velocity, position) in self.velocities.iter_mut().zip(self.positions.iter()) {
            *velocity += self
                .positions
                .iter()
                .map(|other| (*other - *position).signum())
                .sum()
        }
    }

    fn velocity_step(&mut self) {
        for (position, velocity) in self.positions.iter_mut().zip(self.velocities.iter()) {
            *position += *velocity;
        }
    }

    pub fn find_period(&self) -> usize {
        let mut tmp = self.clone();
        for i in 1.. {
            tmp.step();
            if *self == tmp {
                return i;
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State<T> {
    pub x: StateSlice<T>,
    pub y: StateSlice<T>,
    pub z: StateSlice<T>,
}

impl<T> State<T>
where
    T: Integer + Signed + AddAssign + Sum + Copy,
{
    pub fn energy(&self) -> T {
        let pot = self
            .x
            .positions
            .iter()
            .zip(self.y.positions.iter())
            .zip(self.z.positions.iter())
            .map(|((x, y), z)| x.abs() + y.abs() + z.abs());
        let kin = self
            .x
            .velocities
            .iter()
            .zip(self.y.velocities.iter())
            .zip(self.z.velocities.iter())
            .map(|((x, y), z)| x.abs() + y.abs() + z.abs());
        pot.zip(kin).map(|(p, k)| p * k).sum()
    }

    pub fn step(&mut self) {
        self.x.step();
        self.y.step();
        self.z.step();
    }
}

pub struct Simulator<T> {
    state: State<T>,
}

impl<T> Simulator<T>
where
    T: Integer + Signed + AddAssign + Sum + Copy,
{
    pub fn new(state: State<T>) -> Self {
        Simulator { state }
    }

    pub fn state(&self) -> &State<T> {
        &self.state
    }

    pub fn step(&mut self) {
        self.state.step();
    }

    pub fn find_period(&self) -> usize {
        self.state
            .x
            .find_period()
            .lcm(&self.state.y.find_period())
            .lcm(&self.state.z.find_period())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn example1() {
        let mut sim = Simulator::new(State {
            x: StateSlice {
                positions: vec![-1, 2, 4, 3],
                velocities: vec![0; 4],
            },
            y: StateSlice {
                positions: vec![0, -10, -8, 5],
                velocities: vec![0; 4],
            },
            z: StateSlice {
                positions: vec![2, -7, 8, -1],
                velocities: vec![0; 4],
            },
        });
        // Step 1
        sim.step();
        assert_eq!(
            sim.state(),
            &State {
                x: StateSlice {
                    positions: vec![2, 3, 1, 2],
                    velocities: vec![3, 1, -3, -1],
                },
                y: StateSlice {
                    positions: vec![-1, -7, -7, 2],
                    velocities: vec![-1, 3, 1, -3],
                },
                z: StateSlice {
                    positions: vec![1, -4, 5, 0],
                    velocities: vec![-1, 3, -3, 1],
                },
            }
        );
        for _ in 0..9 {
            sim.step();
        }
        assert_eq!(sim.state().energy(), 179);
        let i = sim.find_period();
        assert_eq!(i, 2772);
    }

    #[test]
    fn example2() {
        let mut sim = Simulator::new(State {
            x: StateSlice {
                positions: vec![-8, 5, 2, 9],
                velocities: vec![0; 4],
            },
            y: StateSlice {
                positions: vec![-10, 5, -7, -8],
                velocities: vec![0; 4],
            },
            z: StateSlice {
                positions: vec![0, 10, 3, -3],
                velocities: vec![0; 4],
            },
        });
        for _ in 0..100 {
            sim.step();
        }
        assert_eq!(sim.state().energy(), 1940);
        let i = sim.find_period();
        assert_eq!(i, 4_686_774_924);
    }

    fn read_input(data: &str) -> State<i32> {
        let re = Regex::new(r"<x=((?:-)?\d+), y=((?:-)?\d+), z=((?:-)?\d+)>").unwrap();
        let data: Vec<_> = data
            .lines()
            .map(|line| {
                let caps = re.captures(line).unwrap();
                assert_eq!(caps.len(), 4);
                [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                ]
            })
            .collect();
        State {
            x: StateSlice {
                positions: data.iter().map(|line| line[0]).collect(),
                velocities: vec![0; data.len()],
            },
            y: StateSlice {
                positions: data.iter().map(|line| line[1]).collect(),
                velocities: vec![0; data.len()],
            },
            z: StateSlice {
                positions: data.iter().map(|line| line[2]).collect(),
                velocities: vec![0; data.len()],
            },
        }
    }

    #[test]
    fn day_12_part_1() {
        let mut sim = Simulator::new(read_input(include_str!("input")));
        println!("{:?}", sim.state());
        for _ in 0..1000 {
            sim.step();
        }
        assert_eq!(sim.state().energy(), 7202);
    }

    #[test]
    fn day_12_part_2() {
        let sim = Simulator::new(read_input(include_str!("input")));
        let i = sim.find_period();
        assert_eq!(i, 537_881_600_740_876);
    }
}
