use day_9::computer::Computer;

pub trait TractorBeam {
    fn covers(&self, x: usize, y: usize) -> bool;
}

impl TractorBeam for Vec<isize> {
    fn covers(&self, x: usize, y: usize) -> bool {
        let mut inputs = vec![y, x];
        let mut output = None;
        Computer::new(
            self.clone(),
            || inputs.pop().unwrap() as isize,
            |v| {
                assert!(output.is_none());
                output = Some(v);
            },
        )
        .run()
        .expect("Execution error");
        output.expect("Intcode error") == 1
    }
}

pub fn count_covered(beam: &impl TractorBeam, size: usize) -> usize {
    (0..size)
        .map(|y| (0..size).filter(|&x| beam.covers(x, y)).count())
        .sum()
}

fn fits(beam: &impl TractorBeam, coords: &(usize, usize), size: usize) -> bool {
    let &(x, y) = coords;
    // No need to check bottom right per definition of the beam.
    beam.covers(x, y) && beam.covers(x + size - 1, y) && beam.covers(x, y + size - 1)
}

pub fn find_box(beam: &impl TractorBeam, size: usize, max: usize) -> Option<(usize, usize)> {
    (0..max)
        .flat_map(|y| (0..=y).map(move |x| (x, y)))
        .filter(|coords| fits(beam, coords, size))
        .next()
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

    impl TractorBeam for &str {
        fn covers(&self, x: usize, y: usize) -> bool {
            match self
                .lines()
                .skip(y)
                .next()
                .and_then(|row| row.chars().skip(x).next())
            {
                Some('#') => true,
                _ => false,
            }
        }
    }

    #[test]
    fn small_find() {
        let data = "#..............\n\
                    ...............\n\
                    ...............\n\
                    ..#............\n\
                    ...#...........\n\
                    ....#..........\n\
                    ....##.........\n\
                    .....#.........\n\
                    ......#........\n\
                    ......##.......\n\
                    .......##......\n\
                    ........##.....\n\
                    ........###....\n\
                    .........##....\n\
                    ..........##...";
        assert_eq!(find_box(&data, 2, 13).unwrap(), (8, 11));
    }

    #[test]
    fn part_2() {
        assert_eq!(
            find_box(&str_to_intcode(include_str!("input")), 100, 1000).unwrap(),
            (684, 937)
        );
    }
}
