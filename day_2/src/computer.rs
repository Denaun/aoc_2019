use std::convert::TryFrom;

pub struct Computer {
    pub intcode: Vec<usize>,
    ip: usize,
}

impl Computer {
    pub fn new(intcode: Vec<usize>) -> Computer {
        Computer {
            intcode: intcode,
            ip: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = Op::try_from(self.intcode[self.ip]).unwrap();
            if op == Op::STOP {
                return;
            }
            self.execute(&op);
            self.ip += 1 + op.operands();
        }
    }

    fn execute(&mut self, op: &Op) {
        match op {
            Op::ADD => {
                let fst = self.intcode[self.ip + 1];
                let snd = self.intcode[self.ip + 2];
                let trd = self.intcode[self.ip + 3];
                self.intcode[trd] = self.intcode[fst] + self.intcode[snd]
            }
            Op::MUL => {
                let fst = self.intcode[self.ip + 1];
                let snd = self.intcode[self.ip + 2];
                let trd = self.intcode[self.ip + 3];
                self.intcode[trd] = self.intcode[fst] * self.intcode[snd]
            }
            Op::STOP => std::unreachable!(),
        };
    }
}

#[derive(PartialEq)]
pub enum Op {
    ADD,
    MUL,
    STOP,
}

impl Op {
    pub fn operands(&self) -> usize {
        match self {
            Op::ADD => 3,
            Op::MUL => 3,
            Op::STOP => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromOpError(());

impl TryFrom<usize> for Op {
    type Error = TryFromOpError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Op::ADD),
            2 => Ok(Op::MUL),
            99 => Ok(Op::STOP),
            _ => Err(TryFromOpError(())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut computer = Computer::new(vec![1, 0, 0, 0, 99]);
        computer.run();
        assert_eq!(computer.intcode, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_example2() {
        let mut computer = Computer::new(vec![2, 3, 0, 3, 99]);
        computer.run();
        assert_eq!(computer.intcode, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_example3() {
        let mut computer = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        computer.run();
        assert_eq!(computer.intcode, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_example4() {
        let mut computer = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run();
        assert_eq!(computer.intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_part_1() {
        // Solution for part 1.
        let mut intcode: Vec<usize> = include_str!("input")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        intcode[1] = 15;
        intcode[2] = 03;
        let mut computer = Computer::new(intcode);
        computer.run();
        assert_eq!(computer.intcode[0], 9581917);
    }
}
