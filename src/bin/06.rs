use std::num::ParseIntError;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

struct IndexIterator<'b> {
    cur: Coord,
    start: &'b Coord,
    end: &'b Coord,
}

impl<'b> IndexIterator<'b> {
    fn new(start: &'b Coord, end: &'b Coord) -> IndexIterator<'b> {
        IndexIterator {
            cur: *start,
            start,
            end,
        }
    }
}

impl<'b> Iterator for IndexIterator<'b> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.x > self.end.x {
            self.cur.x = self.start.x;
            self.cur.y += 1;
        }
        if self.cur.x <= self.end.x && self.cur.y <= self.end.y {
            let index = self.cur.y * 1000 + self.cur.x;
            self.cur.x += 1;

            return Some(index);
        }
        None
    }
}

trait Light {
    fn count_on(&self) -> usize;

    fn turn_off(&mut self, start: &Coord, end: &Coord);

    fn turn_on(&mut self, start: &Coord, end: &Coord);

    fn toggle(&mut self, start: &Coord, end: &Coord);
}

struct LightsOne(Vec<bool>);

impl LightsOne {
    fn new() -> Self {
        let vec: Vec<bool> = (0..1_000_000).map(|_| false).collect();
        Self(vec)
    }
    fn iterate(&mut self, start: &Coord, end: &Coord, f: fn(bool) -> bool) {
        let index_iter = IndexIterator::new(start, end);
        for index in index_iter {
            if let Some(l) = self.0.get_mut(index) {
                *l = f(*l);
            }
        }
    }
}

impl Light for LightsOne {
    fn count_on(&self) -> usize {
        let Self(vec) = self;
        vec.iter().filter(|&b| *b).count()
    }

    fn turn_off(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |_| false)
    }

    fn turn_on(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |_| true)
    }

    fn toggle(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |b| !b)
    }

}

#[derive(Debug)]
enum ParseError {
    CommandNotUnderstood,
    NotANumber(ParseIntError),
    InvalidStructure,
}

enum Instruction {
    TurnOn { start: Coord, end: Coord },
    TurnOff { start: Coord, end: Coord },
    Toggle { start: Coord, end: Coord },
}

fn parse_coord(input: &str) -> Result<Coord, ParseError> {
    let (x_str, y_str) = input.split_once(',').ok_or(ParseError::InvalidStructure)?;

    let x = x_str.parse().map_err(ParseError::NotANumber)?;
    let y = y_str.parse().map_err(ParseError::NotANumber)?;

    Ok(Coord { x, y })
}

fn parse_coords(input: &str) -> Result<(Coord, Coord), ParseError> {
    let (first, second) = input
        .split_once(" through ")
        .ok_or(ParseError::InvalidStructure)?;

    Ok((parse_coord(first)?, parse_coord(second)?))
}

fn parse_line(input: &str) -> Result<Instruction, ParseError> {
    if let Some(turn_on) = input.strip_prefix("turn on ") {
        let coords = parse_coords(turn_on)?;
        Ok(Instruction::TurnOn {
            start: coords.0,
            end: coords.1,
        })
    } else if let Some(turn_off) = input.strip_prefix("turn off ") {
        let coords = parse_coords(turn_off)?;
        Ok(Instruction::TurnOff {
            start: coords.0,
            end: coords.1,
        })
    } else if let Some(toggle) = input.strip_prefix("toggle ") {
        let coords = parse_coords(toggle)?;
        Ok(Instruction::Toggle {
            start: coords.0,
            end: coords.1,
        })
    } else {
        Err(ParseError::CommandNotUnderstood)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = LightsOne::new();

    generic_solution(input, &mut lights)
}

struct LightsTwo(Vec<usize>);

impl LightsTwo {
    fn new() -> Self {
        let vec: Vec<usize> = (0..1_000_000).map(|_| 0usize).collect();
        Self(vec)
    }

    fn iterate(&mut self, start: &Coord, end: &Coord, f: fn(usize) -> usize) {
        let index_iter = IndexIterator::new(start, end);
        for index in index_iter {
            if let Some(l) = self.0.get_mut(index) {
                *l = f(*l);
            }
        }
    }
}

impl Light for LightsTwo {
    fn count_on(&self) -> usize {
        let Self(vec) = self;
        vec.iter().sum()
    }

    fn turn_off(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |v| v.saturating_sub(1))
    }

    fn turn_on(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |v| v.saturating_add(1))
    }

    fn toggle(&mut self, start: &Coord, end: &Coord) {
        self.iterate(start, end, |v| v.saturating_add(2))
    }
}

fn generic_solution(input: &str, lights: &mut impl Light) -> Option<usize> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .expect("parses all lines");

    for inst in instructions {
        match inst {
            Instruction::TurnOn { start, end } => lights.turn_on(&start, &end),
            Instruction::TurnOff { start, end } => lights.turn_off(&start, &end),
            Instruction::Toggle { start, end } => lights.toggle(&start, &end),
        }
    }

    Some(lights.count_on())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lights = LightsTwo::new();

    generic_solution(input, &mut lights)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1_000_000)]
    #[case("toggle 0,0 through 999,0", 1_000)]
    #[case("turn on 499,499 through 500,500", 4)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", 1)]
    #[case("toggle 0,0 through 999,999", 2_000_000)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
