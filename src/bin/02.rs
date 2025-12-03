use crate::parser_day2::Range;

advent_of_code::solution!(2);

mod parser_day2 {
    use winnow::{
        Parser,
        ascii::digit1,
        combinator::{separated, separated_pair},
        error::{ContextError, ErrMode},
    };

    #[derive(Debug)]
    pub struct Range {
        pub lower: usize,
        pub higher: usize,
    }

    pub fn parse_input(mut input: &str) -> Vec<Range> {
        splitter(&mut input).unwrap()
    }

    fn splitter(input: &mut &str) -> Result<Vec<Range>, ErrMode<ContextError>> {
        separated(.., range_parser, ',').parse_next(input)
    }

    fn range_parser(input: &mut &str) -> Result<Range, ErrMode<ContextError>> {
        separated_pair(digit1, '-', digit1)
            .parse_next(input)
            .map(|(l, h): (&str, &str)| Range {
                lower: l.parse::<usize>().unwrap(),
                higher: h.parse::<usize>().unwrap(),
            })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let ranges: Vec<Range> = parser_day2::parse_input(input);
    let mut valid_nums: Vec<usize> = Vec::new();
    for range in ranges {
        for value in range.lower..=range.higher {
            let string_version = value.to_string();
            let (left, right) = string_version.split_at(string_version.len() / 2);
            if left == right {
                valid_nums.push(value);
            }
        }
    }
    Some(valid_nums.iter().sum())
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn part_two(input: &str) -> Option<usize> {
    let ranges: Vec<Range> = parser_day2::parse_input(input);
    let mut valid_nums: Vec<usize> = Vec::new();
    for range in ranges {
        for value in range.lower..=range.higher {
            let string_version = value.to_string().into_bytes();
            let valid_chunk_sizes: Vec<usize> = (1..=(string_version.len() / 2))
                .filter(|v| string_version.len() % v == 0)
                .collect();
            let mut valid = false;
            for chunk_size in valid_chunk_sizes {
                let head = string_version.chunks(chunk_size).next().unwrap();
                if string_version.chunks(chunk_size).all(|v| v == head) {
                    valid = true;
                    break;
                }
            }
            if valid {
                valid_nums.push(value);
            }
        }
    }
    Some(valid_nums.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
