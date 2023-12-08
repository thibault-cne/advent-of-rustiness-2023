advent_of_code::solution!(8);

use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, alphanumeric1, line_ending},
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

pub fn part_one(input: &str) -> Option<u64> {
    let (rem, mut directions) = parse_directions(input).expect("parsing error");
    let (_, map) = parse_map(rem).expect("parsing error");
    let mut steps = 0;
    let mut current = "AAA";

    while current != "ZZZ" {
        if directions[0] == Direction::Left {
            current = map[&current].0;
        } else {
            current = map[&current].1;
        };

        directions.rotate_left(1);
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rem, directions) = parse_directions(input).expect("parsing error");
    let (_, map) = parse_map(rem).expect("parsing error");
    let mut cycles = Vec::new();

    map.keys().filter(|k| k.ends_with('A')).for_each(|current| {
        let mut current = current;
        let mut cycle = Vec::new();
        let mut index = 0;
        let mut steps = 0;
        let mut first_z = None;

        loop {
            while steps == 0 || !current.ends_with('Z') {
                steps += 1;
                match directions[index] {
                    Direction::Left => current = &map[current].0,
                    _ => current = &map[current].1,
                }
                index = (index + 1) % directions.len();
            }

            cycle.push(steps);

            if first_z.is_none() {
                first_z = Some(current);
                steps = 0;
            } else if current == first_z.unwrap() {
                break;
            }
        }

        cycles.push(cycle[0]);
    });

    Some(lcm_vec(&cycles) as u64)
}

fn lcm_vec(vec: &[usize]) -> usize {
    let mut result = vec[0];
    for &item in vec.iter().skip(1) {
        result = lcm(result, item);
    }
    result
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }
    a
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (rem, directions) = terminated(is_a("LR"), many1(complete::newline))(input)?;

    Ok((rem, directions.chars().map(Direction::from).collect()))
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let pair = delimited(
        complete::char('('),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        complete::char(')'),
    );
    let line = terminated(separated_pair(alphanumeric1, tag(" = "), pair), line_ending);
    let (rem, items) = many1(line)(input)?;

    let mut map = HashMap::new();

    items.iter().for_each(|&(from, (left, right))| {
        map.insert(from, (left, right));
    });

    Ok((rem, map))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(6));
    }
}
