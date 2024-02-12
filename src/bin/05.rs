use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq)]
enum NaughtyReason {
    NotThreeVowels,
    NoTwiceInARow,
    ForbiddenStrings,
}

fn has_three_vowels(s: &str) -> bool {
    const VOWELS : HashSet<char> = "aeiou".chars().collect();
    s.chars().filter(|c| VOWELS.contains(c)).unique().count() >= 3
}

fn has_a_repeat(s: &str) -> bool {
    s.chars().tuple_windows().any(|(f,s)| f == s)
}

fn has_forbidden_strings(s: &str) -> bool {
    let forbidden : HashSet<(char,char)> = HashSet::from([('a','b'), ('c','d'), ('p','q'), ('x','y')]);

    s.chars().tuple_windows().any(|(f,s)| forbidden.contains(&(f,s)))
}

fn is_nice(s: &str) -> Result<bool, NaughtyReason> {
    if !has_three_vowels(s) {
        return Err(NaughtyReason::NotThreeVowels);
    }

    if !has_a_repeat(s) {
        return Err(NaughtyReason::NoTwiceInARow);
    }

    if has_forbidden_strings(s) {
        return Err(NaughtyReason::ForbiddenStrings);
    }

    Ok(true)
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", Ok(true))]
    #[case("aaa", Ok(true))]
    #[case("jchzalrnumimnmhp", Err(NaughtyReason::NoTwiceInARow))]
    #[case("haegwjzuvuyypxyu", Err(NaughtyReason::ForbiddenStrings))]
    #[case("dvszwmarrgswjxmb", Err(NaughtyReason::NotThreeVowels))]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = is_nice(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("input", 42)]
    fn test_part_two(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
