use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(10);

mod parser_day10 {
    use winnow::{
        Parser,
        ascii::{digit0, line_ending, space0},
        combinator::{delimited, repeat_till, separated, seq},
        error::{ContextError, ErrMode},
        token::{any, take_while},
    };

    pub fn parse_input(mut input: &str) -> Vec<(usize, Vec<usize>)> {
        splitter(&mut input).unwrap()
    }

    fn splitter(input: &mut &str) -> Result<Vec<(usize, Vec<usize>)>, WinnowErr> {
        separated(.., machine_parser, line_ending).parse_next(input)
    }

    fn machine_parser(input: &mut &str) -> Result<(usize, Vec<usize>), WinnowErr> {
        seq!(target_parser,_: space0, buttons_parser,_: space0, _: joltage_parser).parse_next(input)
    }

    fn target_parser(input: &mut &str) -> Result<usize, WinnowErr> {
        delimited('[', hash_parser, ']').parse_next(input)
    }

    fn hash_parser(input: &mut &str) -> Result<usize, WinnowErr> {
        let hash_slice = take_while(.., ('#', '.')).parse_next(input)?;
        Ok(hash_slice
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .fold(0, |acc, (i, _)| acc | 1usize << i))
    }

    type WinnowErr = ErrMode<ContextError>;

    fn buttons_parser(input: &mut &str) -> Result<Vec<usize>, WinnowErr> {
        separated(.., button_parser, ' ').parse_next(input)
    }

    fn button_parser(input: &mut &str) -> Result<usize, WinnowErr> {
        Parser::parse_next(
            &mut delimited('(', separated(.., digit0.parse_to::<usize>(), ','), ')'),
            input,
        )
        .map(|v: Vec<usize>| v.iter().fold(0, |acc, c| acc | 1usize << c))
    }

    fn joltage_parser(input: &mut &str) -> Result<Vec<usize>, WinnowErr> {
        delimited('{', separated(.., digit0.parse_to::<usize>(), ','), '}').parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use crate::parser_day10::{buttons_parser, machine_parser, target_parser};

        #[test]
        fn test_buttons() {
            let mut buttons = "(3) (1,3)";
            let parsed = buttons_parser(&mut buttons).unwrap();
            assert_eq!(parsed[0], 8usize);
            assert_eq!(parsed[1], 10usize);
        }

        #[test]
        fn test_hash() {
            let mut hash = "[.#.#]";
            let parsed = target_parser(&mut hash).unwrap();
            assert_eq!(parsed, 10usize);
        }

        #[test]
        fn test_machine() {
            let mut machine = "[..##] (3) (0,1) {1,2}";
            let parsed = machine_parser(&mut machine).unwrap();
            println!("{:?}", parsed);
            assert_eq!(parsed.0, 12usize);
            assert_eq!(parsed.1[0], 8usize);
            assert_eq!(parsed.1[1], 3usize);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parser_day10::parse_input(input);
    let mut out = 0;
    for machine in machines {
        // machine.0 is the target
        // machine.1 are the buttons
        // we do a bredth first search
        let mut trials: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
        // check for any immediate
        let mut found = false;
        while !found {
            let current = trials.pop_front().unwrap();
            for button in &machine.1 {
                let new = current.0 ^ button;
                if new == machine.0 {
                    out += current.1 + 1;
                    found = true;
                    break;
                }
                trials.push_back((new, current.1 + 1));
            }
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use crate::parser_day10::parse_input;

    use super::*;

    #[test]
    fn test_parser() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let first = result.first().unwrap();
        assert_eq!(6usize, first.0);
        assert_eq!(8usize, first.1[0]);
        assert_eq!(10usize, first.1[1]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
