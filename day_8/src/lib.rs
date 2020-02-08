pub trait Layer {
    type Item;

    fn count(&self, elt: &Self::Item) -> usize;
}

type VecLayer = Vec<Vec<u32>>;
impl Layer for VecLayer {
    type Item = u32;

    fn count(&self, elt: &Self::Item) -> usize {
        self.iter()
            .map(|rows| rows.iter().filter(|v| *v == elt).count())
            .sum()
    }
}

pub trait Image {
    fn read(data: &str, cols: usize, rows: usize) -> Self;
    fn checksum(&self) -> usize;
}

type VecImage = Vec<VecLayer>;
impl Image for VecImage {
    fn read(data: &str, cols: usize, rows: usize) -> Self {
        let mut result = vec![vec![vec![0; cols]; rows]; data.len() / rows / cols];
        for (i, v) in data
            .chars()
            .map(|c: char| c.to_digit(10).unwrap())
            .enumerate()
        {
            result[i / rows / cols][(i / cols) % rows][i % cols] = v;
        }
        result
    }

    fn checksum(&self) -> usize {
        let layer = self.iter().min_by_key(|layer| layer.count(&0)).unwrap();
        layer.count(&1) * layer.count(&2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let image = VecImage::read("123456789012", 3, 2);
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
        let image = VecImage::read(include_str!("input").lines().next().unwrap(), 25, 6);
        assert_eq!(image.checksum(), 1792);
    }
}
