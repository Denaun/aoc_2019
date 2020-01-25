use day_5::computer::Computer;
use log::debug;

pub fn find_largest_output<Phases>(
    intcode: Vec<isize>,
    phase_settings: Phases,
) -> (Vec<isize>, isize)
where
    Phases: Iterator<Item = Vec<isize>>,
{
    phase_settings
        .map(|phases| {
            debug!("Checking phases {:?}", phases);
            let signal = phases.iter().fold(0, |signal, code| {
                // Inputs in reverse order since they are popped.
                let mut inputs = vec![signal, *code];
                let mut signal = None;
                Computer::new(
                    intcode.clone(),
                    || inputs.pop().expect("More than 2 inputs."),
                    |v| {
                        assert!(signal.is_none(), "More than 1 output.");
                        signal = Some(v)
                    },
                )
                .run()
                .unwrap();
                signal.expect("No output.")
            });
            debug!("Resulting signal: {}", signal);
            (phases, signal)
        })
        .max_by_key(|(_phases, signal)| *signal)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use permutator::Permutation;

    #[test]
    fn test_example1() {
        let (phases, signal) = find_largest_output(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            (0..=4).collect::<Vec<isize>>().permutation(),
        );
        assert_eq!(phases, vec![4, 3, 2, 1, 0]);
        assert_eq!(signal, 43210);
    }

    #[test]
    fn test_example2() {
        let (phases, signal) = find_largest_output(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            (0..=4).collect::<Vec<isize>>().permutation(),
        );
        assert_eq!(phases, vec![0, 1, 2, 3, 4]);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn test_example3() {
        let (phases, signal) = find_largest_output(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            (0..=4).collect::<Vec<isize>>().permutation(),
        );
        assert_eq!(phases, vec![1, 0, 4, 3, 2]);
        assert_eq!(signal, 65210);
    }

    #[test]
    fn test_day_7_part_1() {
        // Solution for day 7 part 1.
        let intcode: Vec<isize> = include_str!("input")
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let (_phases, signal) =
            find_largest_output(intcode, (0..=4).collect::<Vec<isize>>().permutation());
        assert_eq!(signal, 20413);
    }
}
