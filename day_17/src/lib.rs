pub mod alignment;

use day_9::computer::Computer;
use std::char;

pub fn get_view(intcode: Vec<isize>) -> String {
    let mut data = String::new();
    Computer::new(
        intcode,
        || unreachable!(),
        |v| {
            data.push(char::from_u32(v as u32).unwrap());
        },
    )
    .run()
    .unwrap();
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_intcode(data: &str) -> Vec<isize> {
        data.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn day_17_part_1() {
        let view = get_view(read_intcode(include_str!("input")));
        let data: Vec<_> = view.lines().collect();
        assert_eq!(alignment::alignment_parameter(&data), 5620);
    }
}
