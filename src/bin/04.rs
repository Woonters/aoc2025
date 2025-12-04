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

type RollData = Option<(usize, Vec<usize>)>;

// if there is an @ at the space we are searching, return the y and x coords
fn check_space(map: &Map, column: usize, row: usize, i: isize, j: isize) -> Option<(usize, usize)> {
    let y: usize = usize::try_from(isize::try_from(column).ok()? + i).ok()?;
    let x: usize = usize::try_from(isize::try_from(row).ok()? + j).ok()?;
    map.get(y)
        .is_some_and(|row_data| row_data.get(x).is_some_and(|c| *c == '@'))
        .then_some((y, x))
}

fn generate_cleaned_map(map: &Map) -> Vec<RollData> {
    let major: usize = map.first().unwrap().len();
    let mut out: Vec<RollData> = vec![None; map.len() * major];

    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if *character == '@' {
                let mut connected: Vec<usize> = Vec::new();
                for i in -1_isize..=1 {
                    for j in -1_isize..=1 {
                        if let Some(coords) = check_space(map, y, x, i, j) {
                            connected.push((major * coords.0) + coords.1);
                        }
                    }
                }
                out[(y * major) + x] = Some((connected.len(), connected));
            }
        }
    }
    out
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Map = parser_day4::parse_input(input);
    let mut cleaned_map: Vec<RollData> = generate_cleaned_map(&map);
    let mut counter = 0;
    loop {
        let reductions: Vec<&Option<(usize, Vec<usize>)>> = cleaned_map
            .iter()
            .filter(|position| {
                position
                    .as_ref()
                    .is_some_and(|(surrounding, _)| *surrounding <= 4)
            })
            .collect();
        if reductions.is_empty() {
            return Some(counter);
        }
        counter += reductions.len();
        let reduction_targets = reductions
            .iter()
            .flat_map(|position| position.as_ref().unwrap().1.clone())
            .collect::<Vec<usize>>();

        for reduction in reduction_targets {
            if let Some((value, _)) = &mut cleaned_map[reduction] {
                *value -= 1;
            }
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
