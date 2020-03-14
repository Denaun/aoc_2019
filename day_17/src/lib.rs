use day_9::computer::Computer;
use std::char;

#[derive(Debug, PartialEq, Eq)]
pub struct Coord(usize, usize);

impl Coord {
    pub fn alignment_parameter(&self) -> usize {
        self.0 * self.1
    }
}

pub fn find_intersections(data: &[&str]) -> Vec<Coord> {
    data.windows(3)
        .enumerate()
        .flat_map(|(y, data)| {
            data[0]
                .as_bytes()
                .windows(3)
                .zip(data[1].as_bytes().windows(3))
                .zip(data[2].as_bytes().windows(3))
                .enumerate()
                .filter_map(move |(x, block)| match block {
                    (([_, b'#', _], b"###"), [_, b'#', _]) => Some(Coord(x + 1, y + 1)),
                    _ => None,
                })
        })
        .collect()
}

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

    #[test]
    fn example_1() {
        let data: Vec<_> = "..#..........\n\
                            ..#..........\n\
                            #######...###\n\
                            #.#...#...#.#\n\
                            #############\n\
                            ..#...#...#..\n\
                            ..#####...^.."
            .lines()
            .collect();
        let params = find_intersections(&data);
        assert_eq!(
            params,
            vec![Coord(2, 2), Coord(2, 4), Coord(6, 4), Coord(10, 4)]
        );
        assert_eq!(
            params
                .iter()
                .map(Coord::alignment_parameter)
                .collect::<Vec<_>>(),
            vec![4, 8, 24, 40]
        );
        assert_eq!(
            params.iter().map(Coord::alignment_parameter).sum::<usize>(),
            76
        );
    }

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
        assert_eq!(
            find_intersections(&data)
                .iter()
                .map(Coord::alignment_parameter)
                .sum::<usize>(),
            5620
        );
    }
}
