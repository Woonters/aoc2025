advent_of_code::solution!(3);
use rayon::prelude::*;

mod parser_day3 {
    use winnow::{
        Parser,
        ascii::line_ending,
        combinator::separated,
        error::{ContextError, ErrMode},
        stream::AsChar,
        token::take_while,
    };

    #[derive(Debug)]
    pub struct Bank {
        pub batteries: Vec<u8>,
    }

    pub fn parse_input(mut input: &str) -> Vec<Bank> {
        splitter(&mut input).unwrap()
    }

    fn splitter(input: &mut &str) -> Result<Vec<Bank>, ErrMode<ContextError>> {
        separated(.., bank_parser, line_ending).parse_next(input)
    }

    fn bank_parser(input: &mut &str) -> Result<Bank, ErrMode<ContextError>> {
        Ok(Bank {
            batteries: take_while(1.., AsChar::is_dec_digit)
                .parse_next(input)?
                .as_bytes()
                .iter()
                .map(|v| v - 48)
                .collect(),
        })
    }
}

// Rayon seems to just slow down each of these, which is a bit of a shame

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    // find the highest number (if we find a 9 we should switch to second part)
    let mut total = 0;
    let banks = parser_day3::parse_input(input);
    for bank in banks {
        let mut index: usize = 0;
        let mut max: u8 = 0;
        let mut max_2: u8 = 0;
        for (i, v) in bank.batteries[..bank.batteries.len() - 1]
            .iter()
            .enumerate()
        {
            if *v == 9 {
                max = 9;
                index = i;
                break;
            }
            if *v > max {
                index = i;
                max = *v;
            }
        }
        for v in &bank.batteries[index + 1..] {
            if *v == 9 {
                max_2 = 9;
                break;
            }
            if *v > max_2 {
                max_2 = *v;
            }
        }
        total += u64::from(max * 10 + max_2);
    }

    Some(total)
}
#[allow(clippy::must_use_candidate)]
pub fn part_one_rayon(input: &str) -> Option<u64> {
    // find the highest number (if we find a 9 we should switch to second part)
    let banks = parser_day3::parse_input(input);
    let total = banks
        .par_iter()
        .map(|bank| {
            let mut index: usize = 0;
            let mut max: u8 = 0;
            let mut max_2: u8 = 0;
            for (i, v) in bank.batteries[..bank.batteries.len() - 1]
                .iter()
                .enumerate()
            {
                if *v == 9 {
                    max = 9;
                    index = i;
                    break;
                }
                if *v > max {
                    index = i;
                    max = *v;
                }
            }
            for v in &bank.batteries[index + 1..] {
                if *v == 9 {
                    max_2 = 9;
                    break;
                }
                if *v > max_2 {
                    max_2 = *v;
                }
            }
            u64::from(max * 10 + max_2)
        })
        .sum();

    Some(total)
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let banks = parser_day3::parse_input(input);
    for bank in banks {
        let mut start = 0;
        let mut end = bank.batteries.len() - 12;
        let mut digits = Vec::with_capacity(12);
        for _ in 0..12 {
            let (value, index) = check_slice(&bank.batteries[start..=end]);
            start += index + 1;
            end += 1;
            digits.push(value);
        }
        total += digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| u64::from(*v) * 10_u64.pow(u32::try_from(i).unwrap()))
            .sum::<u64>();
    }
    Some(total)
}
#[allow(clippy::must_use_candidate, clippy::missing_panics_doc)]
pub fn part_two_rayon(input: &str) -> Option<u64> {
    let banks = parser_day3::parse_input(input);
    let total: u64 = banks
        .par_iter()
        .map(|bank| {
            let mut start = 0;
            let mut end = bank.batteries.len() - 12;
            let mut digits = Vec::with_capacity(12);
            for _ in 0..12 {
                let (value, index) = check_slice(&bank.batteries[start..=end]);
                start += index + 1;
                end += 1;
                digits.push(value);
            }
            digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| u64::from(*v) * 10_u64.pow(u32::try_from(i).unwrap()))
                .sum::<u64>()
        })
        .sum();
    Some(total)
}

fn check_slice(input: &[u8]) -> (u8, usize) {
    let mut index = 0;
    let mut max = 0;
    for (i, v) in input.iter().enumerate() {
        if *v == 9 {
            return (9, i);
        }
        if *v > max {
            max = *v;
            index = i;
        }
    }
    (max, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
