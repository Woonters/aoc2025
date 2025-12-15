use crate::parser_day11::Pool;

advent_of_code::solution!(11);

mod parser_day11 {
    // so let's write out some stuff, we area really doing some form of tree traversal
    // here, we don't have any info on the tree other than it is directional, I am guessing there are no cycles
    // but we might need to acount for that, anway the best way I can think to do this is a two stage one
    // create empty~ish nodes and then go through and fill them with refs to the other nodes, all stored in a pool

    use std::collections::HashMap;

    pub type Pool = HashMap<String, Vec<String>>;

    pub fn parse_input(input: &str) -> Pool {
        let mut pool: Pool = HashMap::new();
        for line in input.lines() {
            let mut split = line.split(':');
            let key = split.next().unwrap();
            let values = split
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .map(|v| v.to_owned())
                .collect();
            pool.insert(key.to_owned(), values);
        }
        pool
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let pool: Pool = parser_day11::parse_input(input);
    // ok so we do a depth first search where we count how many times we get to 'out'
    // we can start looking at caching things later
    let mut search_stack: Vec<String> = Vec::from(["you".to_owned()]);
    let mut out_counter: u64 = 0;
    while let Some(search_node) = search_stack.pop() {
        pool.get(&search_node).unwrap().iter().for_each(|value| {
            if value.as_str() == "out" {
                out_counter += 1;
            } else {
                search_stack.push(value.clone());
            }
        });
    }
    Some(out_counter)
}

// ok so part two needs us to really do something a bit more... recursive
// what about this, all the routes between svr and fft, svr and dac, fft and dac, dac and fft,

fn recursive_search<'a>(
    pool: &'a Pool,
    current_node: String,
    target_node: &String,
    visited: Vec<&'a String>,
) -> Vec<Vec<&'a String>> {
    let mut out: Vec<Vec<&String>> = Vec::new();
    for connection in pool.get(&current_node).unwrap() {
        let mut new_visited = visited.clone();
        new_visited.push(connection);
        if connection == target_node {
            out.push(new_visited);
            continue;
        }
        if connection == "out" {
            continue;
        }
        out.append(&mut recursive_search(
            pool,
            connection.clone(),
            target_node,
            new_visited,
        ));
    }

    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let pool: Pool = parser_day11::parse_input(input);
    let svr_2_fft = dbg!(recursive_search(
        &pool,
        "svr".to_owned(),
        &"fft".to_owned(),
        Vec::new()
    ));

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
