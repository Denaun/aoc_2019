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
    intcode[1] = 12;
    intcode[2] = 02;
    let mut computer = Computer::new(intcode);
    computer.run();
    println!("{}", computer.intcode[0]);
}
