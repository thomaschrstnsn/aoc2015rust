use std::collections::HashSet;

advent_of_code::solution!(3);

type Coord = (i32, i32);

pub fn part_one(input: &str) -> Option<usize> {
    let mut visited : HashSet<Coord> = HashSet::new();
    let mut current : Coord = (0,0);

    visited.insert(current);
    for c in input.chars() {
        match c {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => panic!("unexpected char")
        }

        visited.insert(current);
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited : HashSet<Coord> = HashSet::new();
    let mut santa : Coord = (0,0);
    let mut robot : Coord = (0,0);
    let mut is_it_santa = true;

    visited.insert(santa);
    for c in input.chars() {
        let current = if is_it_santa { &mut santa } else { &mut robot };
        match c {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => panic!("unexpected char")
        }

        is_it_santa = !is_it_santa;

        visited.insert(*current);
    }

    Some(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
