use day_9::computer::Computer;
use std::collections::HashMap;

pub type Coord2D = (isize, isize);

#[derive(Debug, PartialEq)]
pub enum Tile {
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

enum NextInput {
    X,
    Y,
    Tile,
}

pub struct GameFsm {
    pub tiles: HashMap<Coord2D, Tile>,
    next_input: NextInput,
    x: Option<isize>,
    y: Option<isize>,
}

impl GameFsm {
    fn new() -> Self {
        GameFsm {
            tiles: HashMap::new(),
            next_input: NextInput::X,
            x: None,
            y: None,
        }
    }

    fn input(&mut self, v: isize) {
        match self.next_input {
            NextInput::X => {
                self.x = Some(v);
                self.next_input = NextInput::Y;
            }
            NextInput::Y => {
                self.y = Some(v);
                self.next_input = NextInput::Tile;
            }
            NextInput::Tile => {
                let tile = match v {
                    0 => None,
                    1 => Some(Tile::Wall),
                    2 => Some(Tile::Block),
                    3 => Some(Tile::HorizontalPaddle),
                    4 => Some(Tile::Ball),
                    _ => unreachable!(),
                };
                if let Some(tile) = tile {
                    self.tiles.insert((self.x.unwrap(), self.y.unwrap()), tile);
                }
                self.x = None;
                self.y = None;
                self.next_input = NextInput::X;
            }
        }
    }
}

pub fn run_arcade_cabinet(intcode: Vec<isize>) -> GameFsm {
    let mut fsm = GameFsm::new();
    Computer::new(intcode, || unreachable!(), |v| fsm.input(v))
        .run()
        .unwrap();
    fsm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut fsm = GameFsm::new();
        for i in &[1, 2, 3, 6, 5, 4] {
            fsm.input(*i);
        }
        assert_eq!(fsm.tiles.len(), 2);
        assert_eq!(fsm.tiles.get(&(1, 2)), Some(&Tile::HorizontalPaddle));
        assert_eq!(fsm.tiles.get(&(6, 5)), Some(&Tile::Ball));
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
    fn day_13_part_1() {
        let fsm = run_arcade_cabinet(read_intcode(include_str!("input")));
        assert_eq!(
            fsm.tiles
                .iter()
                .filter(|(_, tile)| **tile == Tile::Block)
                .count(),
            280
        );
    }
}
