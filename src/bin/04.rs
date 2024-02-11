use md5::Digest;

advent_of_code::solution!(4);

fn hash(s: &str, n: usize) -> Digest {
    let input = format!("{}{}", s, n);

    md5::compute(input)
}

fn is_valid_advent_coin(d: &Digest, zeros: usize) -> bool {
    format!("{d:x}").chars().take(zeros).all(|c| c == '0')
}

pub fn part_one(input: &str) -> Option<usize> {
    for v in 0usize.. {
        if is_valid_advent_coin(&hash(input, v), 5) {
            return Some(v)
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    for v in 0usize.. {
        if is_valid_advent_coin(&hash(input, v), 6) {
            return Some(v)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043, "000001dbbfa3a5c83a2d506429c7b00e")]
    #[case("pqrstuv", 1048970, "000006136ef2ff3b291c85725f17325c")]
    fn test_hash(#[case] input: &str, #[case] seed: usize, #[case] expected: &str) {
        let result = hash(input, seed);

        assert_eq!(format!("{result:x}"),expected);
    }

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_valid_advent_coin(#[case] input: &str, #[case] seed: usize) {
        let result = is_valid_advent_coin(&hash(input,seed),5);

        assert!(result);
    }

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }
}
