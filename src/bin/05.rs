advent_of_code::solution!(5);

use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    multi::{many0, many_till, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn part_one(input: &str) -> Option<u64> {
    let (remaining, seeds) = Seeds::parse(input).unwrap();
    let (_, map) = many0(Map::parse)(remaining).unwrap();
    let mut ans = u64::MAX;

    for seed in seeds.0 {
        let mut name = "seed";
        let mut value = seed;

        loop {
            if name == "location" {
                break;
            }

            let mapping = map.iter().find(|m| m.source == name).unwrap();

            if let Some((src, dst)) = mapping.mapping.iter().find(|(src, _)| src.contains(&value)) {
                value = dst.start + (value - src.start);
            }
            name = &mapping.destination;
        }

        if value < ans {
            ans = value;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (remaining, seeds) = SeedsRange::parse(input).unwrap();
    let (_, map) = many0(Map::parse)(remaining).unwrap();

    let location = map.iter().find(|m| m.destination == "location").unwrap();
    let max = location
        .mapping
        .iter()
        .map(|(_, dst)| dst.end)
        .max()
        .unwrap();

    let ans = (0..max).find(|&i| {
        let mut name = "location";
        let mut value = i;

        loop {
            if name == "seed" {
                break;
            }

            let mapping = map.iter().find(|m| m.destination == name).unwrap();

            if let Some((src, dst)) = mapping.mapping.iter().find(|(_, dst)| dst.contains(&value)) {
                value = src.start + (value - dst.start);
            }

            name = &mapping.source;
        }

        seeds.0.iter().any(|s| s.contains(&value))
    });

    ans
}

#[derive(Debug)]
struct Seeds(Vec<u64>);

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, seeds) = terminated(
            preceded(tag("seeds: "), separated_list0(space1, digit1)),
            tag("\n\n"),
        )(input)?;

        Ok((
            remaining,
            Self(
                seeds
                    .into_iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug)]
struct SeedsRange(Vec<Range<u64>>);

impl SeedsRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, seeds) = terminated(
            preceded(
                tag("seeds: "),
                separated_list0(space1, separated_pair(digit1, space1, digit1)),
            ),
            tag("\n\n"),
        )(input)?;

        Ok((
            remaining,
            Self(
                seeds
                    .into_iter()
                    .map(|s| {
                        let start = s.0.parse::<u64>().unwrap();
                        let length = s.1.parse::<u64>().unwrap();

                        start..(start + length)
                    })
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    mapping: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, (source, destination)) =
            terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(input)?;

        let list = separated_list0(space1, digit1);
        let (remaining, (mapping, _)) = many_till(terminated(list, newline), newline)(remaining)?;

        let mapping = mapping
            .into_iter()
            .map(|v| {
                let src_start = v[1].parse::<u64>().unwrap();
                let dst_start = v[0].parse::<u64>().unwrap();
                let length = v[2].parse::<u64>().unwrap();

                (
                    src_start..(src_start + length),
                    dst_start..(dst_start + length),
                )
            })
            .collect();

        Ok((
            remaining,
            Self {
                source: source.to_string(),
                destination: destination.to_string(),
                mapping,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_map() {
        let input = "light-to-temperature map:
45 77 23
81 45 19
68 64 13

";

        let map = Map::parse(input);

        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(map.0, "");
    }

    #[test]
    fn test_seeds() {
        let input = "seeds: 1 2 3 4 5

";
        let seeds = Seeds::parse(input);

        assert!(seeds.is_ok());
        let seeds = seeds.unwrap();
        assert_eq!(seeds.1 .0, vec![1, 2, 3, 4, 5]);
    }
}
