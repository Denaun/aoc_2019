use day_5::computer::Computer;
use log::debug;
use std::sync::mpsc;
use std::time::Duration;

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
            let intcode = intcode.clone();

            let (txs, rxs): (Vec<_>, Vec<_>) = phases
                .iter()
                .map(|code| {
                    let (tx, rx) = mpsc::channel();
                    tx.send(*code).unwrap();
                    (tx, rx)
                })
                .unzip();
            txs.first().unwrap().send(0).unwrap(); // Initial input.
            let (signal_tx, signal_rx) = mpsc::channel();

            rayon::scope(move |s| {
                // Use the next channel to transmit.
                for (rx, tx) in rxs.into_iter().zip(txs.into_iter().cycle().skip(1)) {
                    let intcode = intcode.clone();
                    let signal_tx = signal_tx.clone();
                    s.spawn(move |_| {
                        Computer::new(
                            intcode,
                            || {
                                debug!("Receiving");
                                let v = rx.recv_timeout(Duration::from_secs(1)).unwrap();
                                debug!("Received {}", v);
                                v
                            },
                            |v| {
                                debug!("Sending {}", v);
                                let _ = tx.send(v);
                                signal_tx.send(v).unwrap();
                            },
                        )
                        .run()
                        .unwrap();
                    })
                }
            });

            let signal = signal_rx.iter().last().expect("No output");
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
    use std::io::Write;
    use std::thread;

    fn _init() {
        let _ = env_logger::builder()
            .format(|buf, record| writeln!(buf, "{:?}: {}", thread::current().id(), record.args()))
            .try_init();
    }

    #[test]
    fn test_example1() {
        _init();
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

    #[test]
    fn test_example4() {
        let (phases, signal) = find_largest_output(
            vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            (5..=9).collect::<Vec<isize>>().permutation(),
        );
        assert_eq!(phases, vec![9, 8, 7, 6, 5]);
        assert_eq!(signal, 139629729);
    }

    #[test]
    fn test_example5() {
        let (phases, signal) = find_largest_output(
            vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            (5..=9).collect::<Vec<isize>>().permutation(),
        );
        assert_eq!(phases, vec![9, 7, 8, 5, 6]);
        assert_eq!(signal, 18216);
    }

    #[test]
    fn test_day_7_part_2() {
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
            find_largest_output(intcode, (5..=9).collect::<Vec<isize>>().permutation());
        assert_eq!(signal, 3321777);
    }
}
