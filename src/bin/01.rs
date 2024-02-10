advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().filter_map(|c| match c { '(' => Some(1i32), ')' => Some(-1i32), _ => None}).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut c = 0i32;
    for (index, delta) in input.chars().filter_map(|c| match c { '(' => Some(1i32), ')' => Some(-1i32), _ => None}).enumerate() {
        c += delta;
        if c < 0 {
            return Some(index+1);
        }

    }
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    fn test_part_one(#[case] input: &str,
                     #[case] expected: i32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part_two(#[case] input: &str,
                     #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
