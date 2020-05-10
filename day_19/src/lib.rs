use day_9::computer::Computer;

pub fn count_covered(intcode: &Vec<isize>, size: usize) -> usize {
    (0..size)
        .map(|x| {
            (0..size)
                .filter(|y| {
                    let mut inputs = vec![*y, x];
                    let mut output = None;
                    Computer::new(
                        intcode.clone(),
                        || inputs.pop().unwrap() as isize,
                        |v| {
                            assert!(output.is_none());
                            output = Some(v);
                        },
                    )
                    .run()
                    .expect("Execution error");
                    output.expect("Intcode error") == 1
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_intcode(data: &str) -> Vec<isize> {
        data.lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn part_1() {
        assert_eq!(
            count_covered(&str_to_intcode(include_str!("input")), 50),
            217
        );
    }
}
