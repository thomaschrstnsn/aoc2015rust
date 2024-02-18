advent_of_code::solution!(8);

fn unescape_string(input: &str) -> String {
    let input = input.strip_prefix('"').expect("should start with a quote");
    let input = input.strip_suffix('"').expect("should end with a quote");

    let mut result: Vec<char> = Vec::with_capacity(input.len());
    let mut in_escape = false;
    let mut hex_escape: Option<Vec<char>> = None;
    for c in input.chars() {
        if in_escape {
            if let Some(ref mut hex) = hex_escape {
                hex.push(c);

                if hex.len() == 2 {
                    // let hex_str: String = hex.iter().collect();
                    // let c_lit = u8::from_str_radix(&hex_str, 16).expect(&format!("parsing hex: '{}'", hex_str));
                    // result.push(c_lit as char);

                    // TODO: instead of pushing the "right", which is invalid rust utf8, simply
                    // append a bogus value to get something that gives the right result (length
                    // wise)
                    result.push('X');
                    hex_escape = None;
                    in_escape = false;
                }

                continue;
            }

            if c == 'x' {
                hex_escape = Some(Vec::with_capacity(2));
                continue;
            }

            in_escape = false;
            result.push(c);
            continue;
        }

        if c == '\\' {
            in_escape = true;
            continue;
        }

        result.push(c);
    }

    result.iter().collect()
}

fn escape_string(input: &str) -> String {
    let mut result: Vec<char> = Vec::with_capacity(input.len() + 2);
    result.push('"');

    for c in input.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push('"');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            c => {
                result.push(c);
            }
        }
    }

    result.push('"');
    result.iter().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.len().abs_diff(unescape_string(l).len()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.len().abs_diff(escape_string(l).len()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_string_examples() {
        let results: Vec<String> = advent_of_code::template::read_file("examples", DAY)
            .lines()
            .map(unescape_string)
            .collect();
        assert_eq!(results, vec!["", "abc", "aaa\"aaa", "X"]);
    }

    #[test]
    fn test_escape_string_examples() {
        let results: Vec<usize> = advent_of_code::template::read_file("examples", DAY)
            .lines()
            .map(escape_string)
            .map(|s| s.len())
            .collect();
        assert_eq!(results, vec![6, 9, 16, 11]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
