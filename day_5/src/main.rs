#[macro_use]
extern crate clap;
extern crate log;

mod computer;

use clap::{App, Arg};
use std::io::stdin;

use computer::Computer;

fn main() {
    let matches = App::new("day_5")
        .version(&crate_version!()[..])
        .arg(
            Arg::with_name("intcode")
                .help("the Intcode to run")
                .required(true),
        )
        .get_matches();
    env_logger::init();

    let intcode: Vec<isize> = matches
        .value_of("intcode")
        .unwrap()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    Computer::new(
        intcode,
        || {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            buffer.parse().unwrap()
        },
        |v| println!("{}", v),
    )
    .run()
    .unwrap();
}
