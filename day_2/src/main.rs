#[macro_use]
extern crate clap;
mod computer;

use clap::{App, Arg};

use computer::Computer;

fn main() {
    let matches = App::new("day_2")
        .version(&crate_version!()[..])
        .arg(
            Arg::with_name("intcode")
                .help("the Intcode to run")
                .required(true),
        )
        .get_matches();

    let mut intcode: Vec<usize> = matches
        .value_of("intcode")
        .unwrap()
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    for noun in (0..intcode.len()).filter(|x| x % 4 != 0) {
        intcode[1] = noun;
        for verb in (0..intcode.len()).filter(|x| x % 4 != 0) {
            intcode[2] = verb;
            let mut computer = Computer::new(intcode.clone());
            computer.run();
            if computer.intcode[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
    std::unreachable!();
}
