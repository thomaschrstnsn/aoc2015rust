use std::{num::ParseIntError, cmp};

advent_of_code::solution!(2);

struct Dimension(u32, u32, u32);

#[derive(Debug)]
enum ParseError {
    InvalidStructure,
    NotANumber(ParseIntError),
}

fn parse(input: &str) -> Result<Dimension, ParseError> {
    let splits = input.split('x');

    let nums = splits.map(|s| s.parse::<u32>()).collect::<Result<Vec<_>, _>>().map_err(ParseError::NotANumber)?;

    let [h,w,l] = nums[..3] else { return Err(ParseError::InvalidStructure);};

    Ok(Dimension(h,w,l))
}

fn solve_one(dim: &Dimension) -> u32 {
    let Dimension(h,w,l) = dim;
    let min = cmp::min(cmp::min(l*w,w*h), h*l);

    (2*l*w) + (2*w*h) + (2*h*l) + min
}

pub fn part_one(input: &str) -> Option<u32> {
    let dims = input.lines().map(parse).collect::<Result<Vec<_>,_>>().ok()?;

    Some(dims.iter().map(solve_one).sum())
}

fn solve_two(dim: &Dimension) -> u32 {
    let Dimension(h,w,l) = dim;
    let (&min1, &min2) = if cmp::min(h, w) == h {
        (h, cmp::min(w, l))
    } else {
        (w, cmp::min(h, l))
    };

    min1 + min1 + min2 + min2 + (h*w*l)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dims = input.lines().map(parse).collect::<Result<Vec<_>,_>>().ok()?;

    Some(dims.iter().map(solve_two).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_part_two(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
