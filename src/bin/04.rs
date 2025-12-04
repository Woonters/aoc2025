use crate::parser_day4::Map;

advent_of_code::solution!(4);

mod parser_day4 {
    use winnow::{
        Parser,
        ascii::line_ending,
        combinator::separated,
        error::{ContextError, ErrMode},
        token::take_while,
    };

    pub type Map = Vec<Vec<char>>;
    pub fn parse_input(mut input: &str) -> Map {
        line_splitter(&mut input).unwrap()
    }

    fn line_splitter(input: &mut &str) -> Result<Map, ErrMode<ContextError>> {
        separated(.., line_parser, line_ending).parse_next(input)
    }

    fn line_parser(input: &mut &str) -> Result<Vec<char>, ErrMode<ContextError>> {
        take_while(.., ('.', '@'))
            .parse_next(input)
            .map(|s| s.chars().collect())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Map = parser_day4::parse_input(input);
    // for each @ symbol grab all the surrounding elements
    let mut count = 0;
    for (column, data) in map.iter().enumerate() {
        for (row, character) in data.iter().enumerate() {
            if *character == '@' {
                let mut surrounding: u8 = 0;
                for i in -1_isize..=1 {
                    for j in -1_isize..=1 {
                        surrounding += helper(&map, column, row, i, j).unwrap_or(0)
                    }
                }
                if surrounding <= 4 {
                    count += 1
                }
            }
        }
    }
    Some(count)
}

fn helper(map: &Map, column: usize, row: usize, i: isize, j: isize) -> Option<u8> {
    // get the location we will be searching;
    let y: usize = usize::try_from(isize::try_from(column).ok()? + i).ok()?;
    let x: usize = usize::try_from(isize::try_from(row).ok()? + j).ok()?;
    map.get(y).map(|v| {
        v.get(x)
            .map(|c| match *c {
                '@' => 1,
                _ => 0,
            })
            .unwrap_or(0)
    })
}

fn helper2(map: &mut Map) -> usize {
    // for each @ symbol grab all the surrounding elements
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (column, data) in map.iter().enumerate() {
        for (row, character) in data.iter().enumerate() {
            if *character == '@' {
                let mut surrounding: u8 = 0;
                for i in -1_isize..=1 {
                    for j in -1_isize..=1 {
                        surrounding += helper(map, column, row, i, j).unwrap_or(0)
                    }
                }
                if surrounding <= 4 {
                    positions.push((column, row));
                }
            }
        }
    }
    for pos in &positions {
        map[pos.0][pos.1] = '.';
    }
    positions.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: Map = parser_day4::parse_input(input);
    let mut counter = 0;
    loop {
        match helper2(&mut map) {
            0 => return Some(counter),
            x => counter += x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
