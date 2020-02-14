#[macro_use]
extern crate clap;
mod computer;

use clap::{App, Arg};

use computer::find_noun_verb;

fn main() {
    let matches = App::new("day_2")
        .version(&crate_version!()[..])
        .arg(
            Arg::with_name("intcode")
                .help("the Intcode to run")
                .required(true),
        )
        .arg(
            Arg::with_name("result")
                .help("the result to get after running the modified intcode")
                .required(true),
        )
        .get_matches();

    let intcode: Vec<usize> = matches
        .value_of("intcode")
        .unwrap()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    let result = value_t!(matches, "result", usize).unwrap();
    if let Some((noun, verb)) = find_noun_verb(intcode, result) {
        println!("{}", 100 * noun + verb);
    } else {
        println!("No solution found");
    }
}
