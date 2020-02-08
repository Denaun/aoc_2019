#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    pub fn superpose_to(&self, other: &Pixel) -> Pixel {
        match self {
            Pixel::Transparent => *other,
            _ => *self,
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Transparent
    }
}
impl From<u32> for Pixel {
    fn from(v: u32) -> Self {
        match v {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            _ => panic!("Invalid pixel color"),
        }
    }
}

pub trait Layer {
    type Item;

    fn count(&self, elt: &Self::Item) -> usize;
}

type VecLayer<T> = Vec<Vec<T>>;
impl<T> Layer for VecLayer<T>
where
    T: PartialEq,
{
    type Item = T;

    fn count(&self, elt: &Self::Item) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|v| *v == elt).count())
            .sum()
    }
}

pub trait Image {
    fn read(data: &str, cols: usize, rows: usize) -> Self;
    fn checksum(&self) -> usize;
}

type VecImage<T> = Vec<VecLayer<T>>;
impl<T> Image for VecImage<T>
where
    T: Default + Clone + From<u32> + PartialEq,
{
    fn read(data: &str, cols: usize, rows: usize) -> Self {
        let mut result = vec![vec![vec![T::default(); cols]; rows]; data.len() / rows / cols];
        for (i, v) in data
            .chars()
            .map(|c: char| c.to_digit(10).unwrap().into())
            .enumerate()
        {
            result[i / rows / cols][(i / cols) % rows][i % cols] = v;
        }
        result
    }

    fn checksum(&self) -> usize {
        let layer = self
            .iter()
            .min_by_key(|layer| layer.count(&0.into()))
            .unwrap();
        layer.count(&1.into()) * layer.count(&2.into())
    }
}

trait Decode<T>
where
    T: Layer,
{
    fn decode(&self) -> T;
}

impl Decode<VecLayer<Pixel>> for VecImage<Pixel> {
    fn decode(&self) -> VecLayer<Pixel> {
        self.iter().skip(1).fold(self[0].clone(), |acc, layer| {
            acc.into_iter()
                .zip(layer.iter())
                .map(|(acc_row, layer_row)| {
                    acc_row
                        .into_iter()
                        .zip(layer_row.iter())
                        .map(|(acc_pixel, layer_pixel)| acc_pixel.superpose_to(layer_pixel))
                        .collect()
                })
                .collect()
        })
    }
}

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for VecLayer<Pixel> {
    fn draw(&self) -> String {
        self.iter()
            .map(|row| {
                row.iter()
                    .map(|v| match v {
                        Pixel::White => '#',
                        Pixel::Black => ' ',
                        Pixel::Transparent => panic!("Can't draw transparent"),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let image = VecImage::<u32>::read("123456789012", 3, 2);
        assert_eq!(
            image,
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]]
            ]
        );
        assert_eq!(image.checksum(), 1);
    }

    #[test]
    fn day_8_part_1() {
        let image = VecImage::<u32>::read(include_str!("input").lines().next().unwrap(), 25, 6);
        assert_eq!(image.checksum(), 1792);
    }

    #[test]
    fn example_2() {
        let image = VecImage::<Pixel>::read("0222112222120000", 2, 2);
        assert_eq!(
            image,
            vec![
                vec![
                    vec![Pixel::Black, Pixel::Transparent],
                    vec![Pixel::Transparent, Pixel::Transparent]
                ],
                vec![
                    vec![Pixel::White, Pixel::White],
                    vec![Pixel::Transparent, Pixel::Transparent]
                ],
                vec![
                    vec![Pixel::Transparent, Pixel::Transparent],
                    vec![Pixel::White, Pixel::Transparent]
                ],
                vec![
                    vec![Pixel::Black, Pixel::Black],
                    vec![Pixel::Black, Pixel::Black]
                ],
            ]
        );
        assert_eq!(
            image.decode(),
            vec![
                vec![Pixel::Black, Pixel::White],
                vec![Pixel::White, Pixel::Black]
            ],
        );
        assert_eq!(image.decode().draw(), " #\n# ");
    }

    #[test]
    fn day_8_part_2() {
        let image = VecImage::<Pixel>::read(include_str!("input").lines().next().unwrap(), 25, 6);
        assert_eq!(
            image.decode().draw(),
            "\
#      ## ####  ##  #  # 
#       # #    #  # #  # 
#       # ###  #    #### 
#       # #    #    #  # 
#    #  # #    #  # #  # 
####  ##  ####  ##  #  # "
        );
    }
}
