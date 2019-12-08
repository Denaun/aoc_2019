use log::debug;
use snafu::{ensure, ResultExt, Snafu};
use std::convert::TryFrom;

pub struct Computer<R, W>
where
    R: FnMut() -> isize,
    W: FnMut(isize) -> (),
{
    pub intcode: Vec<isize>,
    pub read: R,
    pub write: W,
    ip: usize,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid address {}: {}", address, source))]
    Address {
        address: usize,
        source: std::num::TryFromIntError,
    },

    #[snafu(display("Invalid op-code {}", value))]
    OpCodeInvalid { value: usize },

    #[snafu(display("Too many digits for instruction {:?}: {:?}", instr, digits))]
    AdditionalDigits {
        instr: Instruction,
        digits: Vec<u32>,
    },

    #[snafu(display("Invalid mode {}", digit))]
    ModeInvalid { digit: u32 },
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl<R, W> Computer<R, W>
where
    R: FnMut() -> isize,
    W: FnMut(isize) -> (),
{
    pub fn new(intcode: Vec<isize>, read: R, write: W) -> Computer<R, W> {
        Computer {
            intcode,
            read,
            write,
            ip: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            debug!("Instruction {}", self.ip);
            let instr = Instruction::try_from(
                usize::try_from(self.intcode[self.ip]).context(Address { address: self.ip })?,
            )?;
            if instr == Instruction::Stop {
                return Ok(());
            }
            self.execute(&instr)?;
            self.ip += 1 + instr.operands();
        }
    }

    fn execute(&mut self, instr: &Instruction) -> Result<()> {
        debug!("Execute {:?}", instr);
        match instr {
            Instruction::Add(mode1, mode2) => {
                self.store(3, self.load(1, mode1)? + self.load(2, mode2)?)
            }
            Instruction::Mul(mode1, mode2) => {
                self.store(3, self.load(1, mode1)? * self.load(2, mode2)?)
            }
            Instruction::Input => {
                let value = (self.read)();
                self.store(1, value)
            }
            Instruction::Output(mode) => {
                let value = self.load(1, mode)?;
                Ok((self.write)(value))
            }
            _ => std::unreachable!(),
        }
    }

    fn load(&self, offset: usize, mode: &Mode) -> Result<isize> {
        let address = self.ip + offset;
        debug!("Load {} ({:?} -> {})", address, mode, self.intcode[address]);
        let result = match mode {
            Mode::Position => {
                let destination =
                    usize::try_from(self.intcode[address]).context(Address { address })?;
                debug!("Load from {}", destination);
                self.intcode[destination]
            }
            Mode::Immediate => self.intcode[address],
        };
        debug!("Loaded {}", result);
        Ok(result)
    }

    fn store(&mut self, offset: usize, value: isize) -> Result<()> {
        let address = self.ip + offset;
        debug!("Store {} to {}", value, address);
        let destination = usize::try_from(self.intcode[address]).context(Address { address })?;
        debug!("Store in {}", destination);
        self.intcode[destination] = value;
        Ok(())
    }
}

fn digits(value: usize) -> Vec<u32> {
    if value == 0 {
        return vec![];
    }
    value
        .to_string()
        .chars()
        .rev()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(Mode, Mode),
    Mul(Mode, Mode),
    Input,
    Output(Mode),
    Stop,
}

impl Instruction {
    pub fn inputs(&self) -> usize {
        match self {
            Instruction::Add(_, _) => 2,
            Instruction::Mul(_, _) => 2,
            Instruction::Input => 0,
            Instruction::Output(_) => 1,
            Instruction::Stop => 0,
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Instruction::Add(_, _) => 1,
            Instruction::Mul(_, _) => 1,
            Instruction::Input => 1,
            Instruction::Output(_) => 0,
            Instruction::Stop => 0,
        }
    }

    pub fn operands(&self) -> usize {
        self.inputs() + self.outputs()
    }
}

impl TryFrom<usize> for Instruction {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        debug!("Decode {}", value);
        let opcode = value % 100;
        let digits = digits(value / 100);
        let mode = |digit| {
            digits
                .get(digit)
                .map(|d| Mode::try_from(*d))
                .unwrap_or(Ok(Mode::Position))
        };
        let instr = match opcode {
            1 => Ok(Instruction::Add(mode(0)?, mode(1)?)),
            2 => Ok(Instruction::Mul(mode(0)?, mode(1)?)),
            3 => Ok(Instruction::Input),
            4 => Ok(Instruction::Output(mode(0)?)),
            99 => Ok(Instruction::Stop),
            _ => Err(Error::OpCodeInvalid { value }),
        }?;
        ensure!(
            digits.len() <= instr.inputs(),
            AdditionalDigits { instr, digits }
        );
        Ok(instr)
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<u32> for Mode {
    type Error = Error;

    fn try_from(digit: u32) -> Result<Self, Self::Error> {
        match digit {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(Error::ModeInvalid { digit }),
        }
    }
}

#[cfg(test)]
mod tests {
    pub fn find_noun_verb(mut intcode: Vec<isize>, result: isize) -> Option<(usize, usize)> {
        for noun in (0..intcode.len()).filter(|x| x % 4 != 0) {
            intcode[1] = noun as isize;
            for verb in (0..intcode.len()).filter(|x| x % 4 != 0) {
                intcode[2] = verb as isize;
                let mut computer = Computer::new(
                    intcode.clone(),
                    || std::unreachable!(),
                    |_| std::unreachable!(),
                );
                computer.run().unwrap();
                if computer.intcode[0] == result {
                    return Some((noun, verb));
                }
            }
        }
        None
    }

    use super::*;

    #[test]
    fn test_example1() {
        let mut computer = Computer::new(
            vec![1, 0, 0, 0, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_example2() {
        let mut computer = Computer::new(
            vec![2, 3, 0, 3, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_example3() {
        let mut computer = Computer::new(
            vec![2, 4, 4, 5, 99, 0],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_example4() {
        let mut computer = Computer::new(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day_2_part_1() {
        // Solution for day 2 part 1.
        let mut intcode: Vec<isize> = include_str!("input_day_2")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        intcode[1] = 12;
        intcode[2] = 02;
        let mut computer = Computer::new(intcode, || std::unreachable!(), |_| std::unreachable!());
        computer.run().unwrap();
        assert_eq!(computer.intcode[0], 9581917);
    }

    #[test]
    fn test_day_2_part_2() {
        // Solution for day 2 part 2.
        let intcode: Vec<isize> = include_str!("input_day_2")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let (noun, verb) = find_noun_verb(intcode, 19690720).unwrap();
        assert_eq!(noun, 25);
        assert_eq!(verb, 05);
    }

    #[test]
    fn test_op_codes() -> Result<(), Error> {
        assert_eq!(
            Instruction::try_from(1)?,
            Instruction::Add(Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(2)?,
            Instruction::Mul(Mode::Position, Mode::Position)
        );
        assert_eq!(Instruction::try_from(3)?, Instruction::Input);
        assert_eq!(
            Instruction::try_from(4)?,
            Instruction::Output(Mode::Position)
        );
        assert_eq!(Instruction::try_from(99)?, Instruction::Stop);
        Ok(())
    }

    #[test]
    fn test_modes() -> Result<(), Error> {
        assert_eq!(
            Instruction::try_from(1101)?,
            Instruction::Add(Mode::Immediate, Mode::Immediate)
        );
        assert_eq!(
            Instruction::try_from(1001)?,
            Instruction::Add(Mode::Position, Mode::Immediate)
        );
        assert_eq!(
            Instruction::try_from(102)?,
            Instruction::Mul(Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(104)?,
            Instruction::Output(Mode::Immediate)
        );
        Ok(())
    }

    #[test]
    fn test_day_5_part_1() {
        // Solution for day 5 part 1.
        let intcode: Vec<isize> = include_str!("input_day_5")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let mut input = vec![1];
        let mut output = vec![];
        let mut computer = Computer::new(intcode, || input.pop().unwrap(), |v| output.push(v));
        computer.run().unwrap();
        assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 13787043]);
    }
}
