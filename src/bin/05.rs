use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq)]
enum NaughtyReasonOne {
    NotThreeVowels,
    NoTwiceInARow,
    ForbiddenStrings,
}

fn has_three_vowels(s: &str) -> bool {
    let vowels: HashSet<char> = "aeiou".chars().collect();
    s.chars().filter(|c| vowels.contains(c)).count() >= 3
}

fn has_a_repeat(s: &str) -> bool {
    s.chars().tuple_windows().any(|(f, s)| f == s)
}

fn has_forbidden_strings(s: &str) -> bool {
    let forbidden: HashSet<(char, char)> =
        HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);

    s.chars()
        .tuple_windows()
        .any(|(f, s)| forbidden.contains(&(f, s)))
}

fn is_nice_one(s: &str) -> Result<bool, NaughtyReasonOne> {
    if !has_three_vowels(s) {
        return Err(NaughtyReasonOne::NotThreeVowels);
    }

    if !has_a_repeat(s) {
        return Err(NaughtyReasonOne::NoTwiceInARow);
    }

    if has_forbidden_strings(s) {
        return Err(NaughtyReasonOne::ForbiddenStrings);
    }

    Ok(true)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter_map(|l| is_nice_one(l).ok()).count())
}

#[derive(Debug, PartialEq, Eq)]
enum NaughtyReasonTwo {
    NoRepeatedPair,
    NoRepeatedCharWithAnotherBetween,
}

fn has_a_repeated_pair(s: &str) -> bool {
    let mut seen: HashMap<(char, char), usize> = HashMap::with_capacity(s.len());
    for (index, (c1, c2)) in s.chars().tuple_windows::<(char, char)>().enumerate() {
        if let Some(seen_index) = seen.get(&(c1, c2)) {
            if seen_index.abs_diff(index) > 1 {
                return true;
            }
        } else {
            seen.insert((c1, c2), index);
        }
    }
    false
}

fn has_repeated_pair_with_another_between(s: &str) -> bool {
    s.chars().tuple_windows().any(|(x, y, z)| x == z && x != y)
}

fn is_nice_two(s: &str) -> Result<bool, NaughtyReasonTwo> {
    if !has_a_repeated_pair(s) {
        return Err(NaughtyReasonTwo::NoRepeatedPair);
    }

    if !has_repeated_pair_with_another_between(s) {
        return Err(NaughtyReasonTwo::NoRepeatedCharWithAnotherBetween);
    }

    Ok(true)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter_map(|l| is_nice_two(l).ok()).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", Ok(true))]
    #[case("aaa", Ok(true))]
    #[case("jchzalrnumimnmhp", Err(NaughtyReasonOne::NoTwiceInARow))]
    #[case("haegwjzuvuyypxyu", Err(NaughtyReasonOne::ForbiddenStrings))]
    #[case("dvszwmarrgswjxmb", Err(NaughtyReasonOne::NotThreeVowels))]
    fn test_is_nice_one(#[case] input: &str, #[case] expected: Result<bool, NaughtyReasonOne>) {
        let result = is_nice_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", Ok(true))]
    #[case("xxyxx", Ok(true))]
    #[case(
        "uurcxstgmygtbstg",
        Err(NaughtyReasonTwo::NoRepeatedCharWithAnotherBetween)
    )]
    #[case("aaa", Err(NaughtyReasonTwo::NoRepeatedPair))]
    #[case("ieodomkazucvgmuy", Err(NaughtyReasonTwo::NoRepeatedPair))]
    fn test_is_nice_two(#[case] input: &str, #[case] expected: Result<bool, NaughtyReasonTwo>) {
        let result = is_nice_two(input);
        assert_eq!(result, expected);
    }
}
