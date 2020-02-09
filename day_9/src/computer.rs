use log::debug;
use snafu::{ensure, ResultExt, Snafu};
use std::cell::RefCell;
use std::collections::HashMap;
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
    rb: isize,
    vmem: RefCell<HashMap<usize, isize>>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid address {}: {}", address, source))]
    Address {
        address: usize,
        source: std::num::TryFromIntError,
    },

    #[snafu(display("Invalid instruction pointer {}", ip))]
    IpInvalid { ip: isize },

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
            rb: 0,
            vmem: RefCell::new(HashMap::new()),
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
            if self.execute(&instr)? {
                self.ip += 1 + instr.operands();
            }
        }
    }

    fn execute(&mut self, instr: &Instruction) -> Result<bool> {
        debug!("Execute {:?}", instr);
        match instr {
            Instruction::Add(mode1, mode2, mode3) => {
                self.store(3, self.load(1, mode1)? + self.load(2, mode2)?, mode3)?;
                Ok(true)
            }
            Instruction::Mul(mode1, mode2, mode3) => {
                self.store(3, self.load(1, mode1)? * self.load(2, mode2)?, mode3)?;
                Ok(true)
            }
            Instruction::Input(mode) => {
                let value = (self.read)();
                self.store(1, value, mode)?;
                Ok(true)
            }
            Instruction::Output(mode) => {
                let value = self.load(1, mode)?;
                (self.write)(value);
                Ok(true)
            }
            Instruction::JumpIfTrue(mode1, mode2) => {
                if self.load(1, mode1)? != 0 {
                    self.ip = self.check_ip(self.load(2, mode2)?)?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Instruction::JumpIfFalse(mode1, mode2) => {
                if self.load(1, mode1)? == 0 {
                    self.ip = self.check_ip(self.load(2, mode2)?)?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Instruction::LessThan(mode1, mode2, mode3) => {
                self.store(
                    3,
                    if self.load(1, mode1)? < self.load(2, mode2)? {
                        1
                    } else {
                        0
                    },
                    mode3,
                )?;
                Ok(true)
            }
            Instruction::Equals(mode1, mode2, mode3) => {
                self.store(
                    3,
                    if self.load(1, mode1)? == self.load(2, mode2)? {
                        1
                    } else {
                        0
                    },
                    mode3,
                )?;
                Ok(true)
            }
            Instruction::RelativeBase(mode) => {
                self.rb += self.load(1, mode)?;
                Ok(true)
            }
            Instruction::Stop => std::unreachable!(),
        }
    }

    fn check_ip(&self, raw_ip: isize) -> Result<usize> {
        if let Ok(ip) = usize::try_from(raw_ip) {
            if ip > self.intcode.len() {
                Err(Error::IpInvalid { ip: raw_ip })
            } else {
                Ok(ip)
            }
        } else {
            Err(Error::IpInvalid { ip: raw_ip })
        }
    }

    fn load(&self, offset: usize, mode: &Mode) -> Result<isize> {
        let address = self.ip + offset;
        let address = self.try_resolve(address, mode)?.unwrap_or(address);
        let value = self.get_mem(address);
        debug!("Loaded {} from {}", value, address);
        Ok(value)
    }

    fn store(&mut self, offset: usize, value: isize, mode: &Mode) -> Result<()> {
        let address = self.ip + offset;
        let address = self.try_resolve(address, mode)?.unwrap();
        debug!("Store {} in {}", value, address);
        *self.get_mem_mut(address) = value;
        Ok(())
    }

    fn try_resolve(&self, address: usize, mode: &Mode) -> Result<Option<usize>> {
        debug!("Resolve {} ({:?})", address, mode);
        let base = match mode {
            Mode::Position => 0,
            Mode::Immediate => return Ok(None),
            Mode::Relative => self.rb,
        };
        let offset = self.get_mem(address);
        let address = usize::try_from(base + offset).context(Address { address })?;
        debug!("Base {}, offset {} => {}", base, offset, address);
        Ok(Some(address))
    }

    fn get_mem(&self, address: usize) -> isize {
        if let Some(result) = self.intcode.get(address) {
            *result
        } else {
            *self.vmem.borrow_mut().entry(address).or_insert(0)
        }
    }

    fn get_mem_mut(&mut self, address: usize) -> &mut isize {
        if let Some(result) = self.intcode.get_mut(address) {
            result
        } else {
            self.vmem.get_mut().entry(address).or_insert(0)
        }
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
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    RelativeBase(Mode),
    Stop,
}

impl Instruction {
    pub fn inputs(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 2,
            Instruction::Mul(_, _, _) => 2,
            Instruction::Input(_) => 0,
            Instruction::Output(_) => 1,
            Instruction::JumpIfTrue(_, _) => 2,
            Instruction::JumpIfFalse(_, _) => 2,
            Instruction::LessThan(_, _, _) => 2,
            Instruction::Equals(_, _, _) => 2,
            Instruction::RelativeBase(_) => 1,
            Instruction::Stop => 0,
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 1,
            Instruction::Mul(_, _, _) => 1,
            Instruction::Input(_) => 1,
            Instruction::Output(_) => 0,
            Instruction::JumpIfTrue(_, _) => 0,
            Instruction::JumpIfFalse(_, _) => 0,
            Instruction::LessThan(_, _, _) => 1,
            Instruction::Equals(_, _, _) => 1,
            Instruction::RelativeBase(_) => 0,
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
            1 => Ok(Instruction::Add(mode(0)?, mode(1)?, mode(2)?)),
            2 => Ok(Instruction::Mul(mode(0)?, mode(1)?, mode(2)?)),
            3 => Ok(Instruction::Input(mode(0)?)),
            4 => Ok(Instruction::Output(mode(0)?)),
            5 => Ok(Instruction::JumpIfTrue(mode(0)?, mode(1)?)),
            6 => Ok(Instruction::JumpIfFalse(mode(0)?, mode(1)?)),
            7 => Ok(Instruction::LessThan(mode(0)?, mode(1)?, mode(2)?)),
            8 => Ok(Instruction::Equals(mode(0)?, mode(1)?, mode(2)?)),
            9 => Ok(Instruction::RelativeBase(mode(0)?)),
            99 => Ok(Instruction::Stop),
            _ => Err(Error::OpCodeInvalid { value }),
        }?;
        ensure!(
            digits.len() <= instr.operands(),
            AdditionalDigits { instr, digits }
        );
        Ok(instr)
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<u32> for Mode {
    type Error = Error;

    fn try_from(digit: u32) -> Result<Self, Self::Error> {
        match digit {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(Error::ModeInvalid { digit }),
        }
    }
}

#[cfg(test)]
mod tests {
    use log::info;

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
    fn example1() {
        let mut computer = Computer::new(
            vec![1, 0, 0, 0, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example2() {
        let mut computer = Computer::new(
            vec![2, 3, 0, 3, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example3() {
        let mut computer = Computer::new(
            vec![2, 4, 4, 5, 99, 0],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example4() {
        let mut computer = Computer::new(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            || std::unreachable!(),
            |_| std::unreachable!(),
        );
        computer.run().unwrap();
        assert_eq!(computer.intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn day_2_part_1() {
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
    fn day_2_part_2() {
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
    fn op_codes() -> Result<(), Error> {
        assert_eq!(
            Instruction::try_from(1)?,
            Instruction::Add(Mode::Position, Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(2)?,
            Instruction::Mul(Mode::Position, Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(3)?,
            Instruction::Input(Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(4)?,
            Instruction::Output(Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(5)?,
            Instruction::JumpIfTrue(Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(6)?,
            Instruction::JumpIfFalse(Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(7)?,
            Instruction::LessThan(Mode::Position, Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(8)?,
            Instruction::Equals(Mode::Position, Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(9)?,
            Instruction::RelativeBase(Mode::Position)
        );
        assert_eq!(Instruction::try_from(99)?, Instruction::Stop);
        Ok(())
    }

    #[test]
    fn modes() -> Result<(), Error> {
        assert_eq!(
            Instruction::try_from(21101)?,
            Instruction::Add(Mode::Immediate, Mode::Immediate, Mode::Relative)
        );
        assert_eq!(
            Instruction::try_from(1001)?,
            Instruction::Add(Mode::Position, Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(102)?,
            Instruction::Mul(Mode::Immediate, Mode::Position, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(104)?,
            Instruction::Output(Mode::Immediate)
        );
        assert_eq!(
            Instruction::try_from(105)?,
            Instruction::JumpIfTrue(Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(1006)?,
            Instruction::JumpIfFalse(Mode::Position, Mode::Immediate)
        );
        assert_eq!(
            Instruction::try_from(1107)?,
            Instruction::LessThan(Mode::Immediate, Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(1108)?,
            Instruction::Equals(Mode::Immediate, Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            Instruction::try_from(209)?,
            Instruction::RelativeBase(Mode::Relative)
        );
        Ok(())
    }

    #[test]
    fn example5() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input == 8 { 1 } else { 0 });
        }
    }

    #[test]
    fn example6() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input < 8 { 1 } else { 0 });
        }
    }

    #[test]
    fn example7() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input == 8 { 1 } else { 0 });
        }
    }

    #[test]
    fn example8() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input < 8 { 1 } else { 0 });
        }
    }

    #[test]
    fn example9() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input != 0 { 1 } else { 0 });
        }
    }

    #[test]
    fn example10() {
        for input in 0..10 {
            let mut output = 0;
            Computer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(output, if input != 0 { 1 } else { 0 });
        }
    }

    #[test]
    fn example11() {
        for input in 0..10 {
            debug!("Input {}", input);
            let mut output = 0;
            Computer::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                || input,
                |v| output = v,
            )
            .run()
            .unwrap();
            assert_eq!(
                output,
                if input < 8 {
                    999
                } else if input == 8 {
                    1000
                } else {
                    1001
                }
            );
        }
    }

    #[test]
    fn day_5_part_1() {
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
        Computer::new(
            intcode,
            || input.pop().unwrap(),
            |v| {
                info!("Write {}", v);
                output.push(v)
            },
        )
        .run()
        .unwrap();
        assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 13787043]);
    }

    #[test]
    fn day_5_part_2() {
        // Solution for day 5 part 1.
        let intcode: Vec<isize> = include_str!("input_day_5")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let mut input = vec![5];
        let mut output = vec![];
        Computer::new(
            intcode,
            || input.pop().unwrap(),
            |v| {
                info!("Write {}", v);
                output.push(v)
            },
        )
        .run()
        .unwrap();
        assert_eq!(output, vec![3892695]);
    }

    #[test]
    fn example12() {
        let intcode = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut output = Vec::with_capacity(intcode.len());
        Computer::new(intcode.clone(), || panic!("No output"), |v| output.push(v))
            .run()
            .unwrap();
        assert_eq!(output, intcode);
    }

    #[test]
    fn example13() {
        let intcode = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut output = 0;
        Computer::new(intcode.clone(), || panic!("No output"), |v| output = v)
            .run()
            .unwrap();
        assert!(output > (1e15 as isize));
        assert!(output < (1e16 as isize));
    }

    #[test]
    fn example14() {
        let intcode = vec![104, 1125899906842624, 99];
        let mut output = 0;
        Computer::new(intcode.clone(), || panic!("No output"), |v| output = v)
            .run()
            .unwrap();
        assert_eq!(output, intcode[1]);
    }

    #[test]
    fn day_9_part_1() {
        // Solution for day 9 part 1.
        let intcode: Vec<isize> = include_str!("input_day_9")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let mut input = vec![1];
        let mut output = vec![];
        Computer::new(
            intcode,
            || input.pop().unwrap(),
            |v| {
                info!("Write {}", v);
                output.push(v)
            },
        )
        .run()
        .unwrap();
        assert_eq!(output, vec![3601950151]);
    }
}
