advent_of_code::solution!(1);

mod parser_day1 {
    use winnow::{
        Parser,
        ascii::line_ending,
        combinator::{dispatch, fail, separated},
        error::{ContextError, ErrMode},
        prelude::*,
        token::{take, take_while},
    };

    pub fn parse_input(mut input: &str) -> Vec<isize> {
        splitter(&mut input).unwrap()
    }

    fn splitter(input: &mut &str) -> Result<Vec<isize>, ErrMode<ContextError>> {
        separated(.., rotation, line_ending).parse_next(input)
    }

    fn rotation(input: &mut &str) -> ModalResult<isize> {
        dispatch! {take(1u16);
            "L" => take_while(.., '0'..='9').try_map(|v: &str| v.parse::<isize>().map(|w| -w)),
            "R" => take_while(.., '0'..='9').try_map(|v: &str| v.parse::<isize>()),
            _ => fail::<_,isize,_>,
        }
        .parse_next(input)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let turns: Vec<isize> = parser_day1::parse_input(input);
    let mut ticker = 50;
    let mut counter = 0;
    for turn in turns {
        ticker += turn;
        if ticker % 100 == 0 {
            counter += 1;
        }
    }
    Some(counter)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let turns: Vec<isize> = parser_day1::parse_input(input);
    let mut ticker = 50;
    let mut counter = 0;
    for turn in turns {
        ticker += turn;
        match ticker {
            ..=0 => {
                counter -= (ticker / 100) - 1;
                ticker = 100 + (ticker % 100);
            }
            100.. => {
                counter += ticker / 100;
                ticker %= 100;
            }
            _ => {}
        }
    }
    #[allow(clippy::cast_sign_loss)] // with well-formed inputs, this should always be positive
    Some(counter as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
