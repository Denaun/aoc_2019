fn fft_pattern<T>(base: &[T], digit: usize) -> impl Iterator<Item = &T> {
    base.iter()
        .flat_map(move |d| std::iter::repeat(d).take(digit + 1))
        .cycle()
        .skip(1)
}

pub fn fft(input: &[i32], pattern: &[i32]) -> Vec<i32> {
    (0..input.len())
        .map(|digit| {
            input
                .iter()
                .zip(fft_pattern(pattern, digit))
                .map(|(x, y)| x * y)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

/// Algorithm from [u/paul2718](https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/fb4bvw4/).
pub fn decode(input: &[i32]) -> i32 {
    const REPS: usize = 10_000;
    const PHASES: usize = 100;

    let start = input[0..7].iter().fold(0, |offset, d| offset * 10 + d) as usize;
    let end = input.len() * REPS;
    assert!(start > end / 2);
    assert!(start < end);
    let mut data = Vec::with_capacity(end - start);
    for i in start..end {
        data.push(input[i % input.len()]);
    }

    for _ in 0..PHASES {
        for idx in (0..data.len() - 1).rev() {
            data[idx] = (data[idx] + data[idx + 1]) % 10;
        }
    }

    data[0..8].iter().fold(0, |offset, d| offset * 10 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

    #[test]
    fn fft_pattern_first_digit() {
        assert_eq!(
            fft_pattern(&[1, 2, 3], 0)
                .take(5)
                .copied()
                .collect::<Vec<_>>(),
            vec![2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn fft_pattern_second_digit() {
        assert_eq!(
            fft_pattern(&[1, 2, 3], 1)
                .take(5)
                .copied()
                .collect::<Vec<_>>(),
            vec![1, 2, 2, 3, 3]
        );
    }

    #[test]
    fn base_fft_pattern() {
        assert_eq!(
            fft_pattern(&BASE_PATTERN, 7)
                .take(40)
                .copied()
                .collect::<Vec<_>>(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1,
                -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ]
        );
    }
    #[test]
    fn example_1() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let phase_1 = fft(&input, &BASE_PATTERN);
        assert_eq!(phase_1, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let phase_2 = fft(&phase_1, &BASE_PATTERN);
        assert_eq!(phase_2, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        let phase_3 = fft(&phase_2, &BASE_PATTERN);
        assert_eq!(phase_3, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        let phase_4 = fft(&phase_3, &BASE_PATTERN);
        assert_eq!(phase_4, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    fn parse_input(data: &str) -> Vec<i32> {
        data.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }

    #[test]
    fn example_2() {
        let mut input = parse_input("80871224585914546619083218645595");
        for _ in 0..100 {
            input = fft(&input, &BASE_PATTERN);
        }
        input.truncate(8);
        assert_eq!(input, parse_input("24176176"));
    }

    #[test]
    fn example_3() {
        let mut input = parse_input("19617804207202209144916044189917");
        for _ in 0..100 {
            input = fft(&input, &BASE_PATTERN);
        }
        input.truncate(8);
        assert_eq!(input, parse_input("73745418"));
    }

    #[test]
    fn example_4() {
        let mut input = parse_input("69317163492948606335995924319873");
        for _ in 0..100 {
            input = fft(&input, &BASE_PATTERN);
        }
        input.truncate(8);
        assert_eq!(input, parse_input("52432133"));
    }

    #[test]
    fn day_16_part_1() {
        let mut input = parse_input(include_str!("input").lines().take(1).next().unwrap());
        for _ in 0..100 {
            input = fft(&input, &BASE_PATTERN);
        }
        input.truncate(8);
        assert_eq!(input, parse_input("68317988"));
    }

    #[test]
    fn example_5() {
        let input = parse_input("03036732577212944063491565474664");
        let output = decode(&input);
        assert_eq!(output, 84_462_026);
    }

    #[test]
    fn example_6() {
        let input = parse_input("02935109699940807407585447034323");
        let output = decode(&input);
        assert_eq!(output, 78_725_270);
    }

    #[test]
    fn example_7() {
        let input = parse_input("03081770884921959731165446850517");
        let output = decode(&input);
        assert_eq!(output, 53_553_731);
    }

    #[test]
    fn day_16_part_2() {
        let input = parse_input(include_str!("input").lines().take(1).next().unwrap());
        let output = decode(&input);
        assert_eq!(output, 53_850_800);
    }
}
