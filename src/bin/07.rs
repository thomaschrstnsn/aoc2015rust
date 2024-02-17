use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, Shl, Shr},
};

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq)]
enum ValueProvider<'a> {
    Id(Identifier<'a>),
    Constant(u16),
}

type Identifier<'a> = &'a str;

#[derive(Debug, Eq, PartialEq)]
enum CircuitElement<'a> {
    Signal {
        source: ValueProvider<'a>,
        output: Identifier<'a>,
    },
    And {
        source1: ValueProvider<'a>,
        source2: ValueProvider<'a>,
        output: Identifier<'a>,
    },
    Or {
        source1: ValueProvider<'a>,
        source2: ValueProvider<'a>,
        output: Identifier<'a>,
    },
    LeftShift {
        source: ValueProvider<'a>,
        shift: u8,
        output: Identifier<'a>,
    },
    RightShift {
        source: ValueProvider<'a>,
        shift: u8,
        output: Identifier<'a>,
    },
    Not {
        source: ValueProvider<'a>,
        output: Identifier<'a>,
    },
}

fn parse_valueprovider(input: &str) -> ValueProvider {
    if let Ok(c) = input.parse::<u16>() {
        ValueProvider::Constant(c)
    } else {
        ValueProvider::Id(input)
    }
}

fn parse_line(input: &str) -> CircuitElement {
    let (before, after) = input.split_once(" -> ").expect("contains arrow");

    let dest = after;

    let before_splits = before.split_whitespace().collect::<Vec<_>>();
    if before_splits.len() == 3 {
        let result = match before_splits[1] {
            "AND" => CircuitElement::And {
                source1: parse_valueprovider(before_splits[0]),
                source2: parse_valueprovider(before_splits[2]),
                output: dest,
            },
            "OR" => CircuitElement::Or {
                source1: parse_valueprovider(before_splits[0]),
                source2: parse_valueprovider(before_splits[2]),
                output: dest,
            },
            "LSHIFT" => CircuitElement::LeftShift {
                source: parse_valueprovider(before_splits[0]),
                shift: before_splits[2].parse::<u8>().expect("parsable shift"),
                output: dest,
            },
            "RSHIFT" => CircuitElement::RightShift {
                source: parse_valueprovider(before_splits[0]),
                shift: before_splits[2].parse::<u8>().expect("parsable shift"),
                output: dest,
            },
            unexpected => panic!("unexpected operation: {}", unexpected),
        };

        return result;
    }

    if before_splits.len() == 2 && before_splits[0] == "NOT" {
        return CircuitElement::Not {
            source: parse_valueprovider(before_splits[1]),
            output: dest,
        };
    }

    if before_splits.len() == 1 {
        return CircuitElement::Signal {
            source: parse_valueprovider(before_splits[0]),
            output: dest,
        };
    }

    panic!("could not parse {input}");
}

fn parse(input: &str) -> Vec<CircuitElement> {
    input.lines().map(parse_line).collect()
}

struct Circuit<'a>(HashMap<Identifier<'a>, u16>);

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn new_from_elements(elements: &[CircuitElement<'a>]) -> Self {
        let mut circuit = Self::new();
        circuit.add_elements(&elements.iter().collect::<Vec<_>>());

        circuit
    }

    fn get(&self, vp: &ValueProvider) -> Option<u16> {
        match vp {
            ValueProvider::Constant(c) => Some(*c),
            ValueProvider::Id(id) => {
                let v = self.0.get(id)?;
                Some(*v)
            }
        }
    }

    fn add_elements(&mut self, elements: &[&CircuitElement<'a>]) {
        let mut retry: Vec<&CircuitElement<'a>> = Vec::with_capacity(elements.len());
        for e in elements {
            if self.add_element(e).is_none() {
                retry.push(e);
            }
        }
        if !retry.is_empty() {
            if elements.len() == retry.len() {
                panic!(
                    "after one round, no elements where succesfully added: {}",
                    elements.len()
                );
            }
            self.add_elements(&retry);
        }
    }

    fn add_element(&mut self, element: &CircuitElement<'a>) -> Option<()> {
        let (value, output) = match element {
            CircuitElement::Signal { source, output } => {
                let v = self.get(source)?;
                (v, *output)
            }
            CircuitElement::And {
                source1,
                source2,
                output,
            } => {
                let v1 = self.get(source1)?;
                let v2 = self.get(source2)?;
                (v1.bitand(v2), *output)
            }
            CircuitElement::Or {
                source1,
                source2,
                output,
            } => {
                let v1 = self.get(source1)?;
                let v2 = self.get(source2)?;
                (v1.bitor(v2), *output)
            }
            CircuitElement::LeftShift {
                source,
                shift,
                output,
            } => {
                let v = self.get(source)?;
                (v.shl(shift), *output)
            }
            CircuitElement::RightShift {
                source,
                shift,
                output,
            } => {
                let v = self.get(source)?;
                (v.shr(shift), *output)
            }
            CircuitElement::Not { source, output } => {
                let v = self.get(source)?;
                (!v, *output)
            }
        };

        self.0.insert(output, value);
        Some(())
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let elements = parse(input);
    let circuit = Circuit::new_from_elements(&elements);

    Some(
        *circuit
            .0
            .get(&"a")
            .expect("should have a wire a with signal"),
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut elements = parse(input);
    let circuit = Circuit::new_from_elements(&elements);

    let a = *circuit
        .0
        .get(&"a")
        .expect("should have a wire a with signal");

    let b_pos = elements
        .iter()
        .position(|e| match e {
            CircuitElement::Signal { source: _, output } => output == &"b",
            _ => false,
        })
        .expect("should have something initializing b");

    *elements.get_mut(b_pos).unwrap() = CircuitElement::Signal { source: ValueProvider::Constant(a), output: "b" };

    let circuit2 = Circuit::new_from_elements(&elements);

    let a2 = *circuit2
        .0
        .get(&"a")
        .expect("should have a wire a with signal");

    Some(a2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    use ValueProvider::{Constant, Id};

    #[rstest]
    #[case("123 -> x", CircuitElement::Signal{ source: Constant(123), output: "x"})]
    #[case("le -> x", CircuitElement::Signal{ source: Id("le"), output: "x"})]
    #[case("x AND y -> d", CircuitElement::And{ source1: Id("x"), source2: Id("y"), output: "d"})]
    #[case("x LSHIFT 2 -> f", CircuitElement::LeftShift{ source: Id("x"), shift: 2, output: "f"})]
    #[case("NOT x -> h", CircuitElement::Not{ source: Id("x"), output: "h"})]
    fn test_parse(#[case] input: &str, #[case] expected: CircuitElement) {
        let actual = parse_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_one() {
        let contents = advent_of_code::template::read_file("examples", DAY);
        let elements = parse(&contents);
        let circuit = Circuit::new_from_elements(&elements);

        let result = circuit.0;

        let expected = vec![
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
