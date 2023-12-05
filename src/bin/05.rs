advent_of_code::solution!(5);

use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::{many0, many1, separated_list0},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn part_one(input: &str) -> Option<u64> {
    let (remaining, seeds) = Seeds::parse(input).unwrap();
    let (_, map) = many0(Map::parse)(remaining).unwrap();

    let location = seeds
        .0
        .into_iter()
        .map(|seed| map.iter().fold(seed, |seed, map| map.translate(seed)))
        .min()
        .unwrap();

    Some(location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (remaining, seeds) = SeedsRange::parse(input).unwrap();
    let (_, map) = many0(Map::parse)(remaining).unwrap();

    let max = map
        .iter()
        .last()
        .unwrap()
        .mapping
        .iter()
        .map(|(_, dst)| dst.end)
        .max()
        .unwrap();

    let location = (0..max).find(|&loc| {
        let seed = map
            .iter()
            .rev()
            .fold(loc, |loc, map| map.rev_translate(loc));
        seeds.0.iter().any(|s| s.contains(&seed))
    });

    location
}

#[derive(Debug)]
struct Seeds(Vec<u64>);

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, seeds) =
            preceded(tag("seeds: "), separated_list0(space1, complete::u64))(input)?;

        Ok((remaining, Self(seeds)))
    }
}

#[derive(Debug)]
struct SeedsRange(Vec<Range<u64>>);

impl SeedsRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, seeds) = preceded(
            tag("seeds: "),
            separated_list0(
                space1,
                separated_pair(complete::u64, tag(" "), complete::u64),
            ),
        )(input)?;

        Ok((
            remaining,
            Self(
                seeds
                    .into_iter()
                    .map(|(start, length)| start..(start + length))
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug)]
struct Map {
    mapping: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn translate(&self, source: u64) -> u64 {
        if let Some((src, dst)) = self.mapping.iter().find(|(src, _)| src.contains(&source)) {
            dst.start + (source - src.start)
        } else {
            source
        }
    }

    fn rev_translate(&self, destination: u64) -> u64 {
        if let Some((src, dst)) = self
            .mapping
            .iter()
            .find(|(_, dst)| dst.contains(&destination))
        {
            src.start + (destination - dst.start)
        } else {
            destination
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, _) = take_until("map:")(input)?;

        let line = tuple((
            complete::u64,
            preceded(tag(" "), complete::u64),
            preceded(tag(" "), complete::u64),
        ));
        let (remaining, mapping) = preceded(tag("map:"), many1(preceded(line_ending, line)))(
            remaining,
        )
        .map(|(remaining, mapping)| {
            (
                remaining,
                mapping
                    .into_iter()
                    .map(|(end, start, length)| (start..(start + length), end..(end + length)))
                    .collect::<Vec<_>>(),
            )
        })?;

        Ok((remaining, Self { mapping }))
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
68 64 13";

        let map = Map::parse(input);

        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(map.0, "");
    }

    #[test]
    fn test_seeds() {
        let input = "seeds: 1 2 3 4 5";
        let seeds = Seeds::parse(input);

        assert!(seeds.is_ok());
        let seeds = seeds.unwrap();
        assert_eq!(seeds.1 .0, vec![1, 2, 3, 4, 5]);
    }
}
