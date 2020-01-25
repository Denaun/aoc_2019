#[macro_use]
extern crate clap;
extern crate day_5;
extern crate log;
extern crate permutator;

mod amplification;

use amplification::find_largest_output;
use clap::{App, Arg};
use permutator::Permutation;

fn main() {
    let matches = App::new("day_7")
        .version(&crate_version!()[..])
        .arg(
            Arg::with_name("intcode")
                .help("the Amplifier Controller Software to run")
                .required(true),
        )
        .get_matches();
    env_logger::init();

    let intcode: Vec<isize> = matches
        .value_of("intcode")
        .unwrap()
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    println!(
        "{:?}",
        find_largest_output(intcode, (0..=4).collect::<Vec<isize>>().permutation())
    );
}
