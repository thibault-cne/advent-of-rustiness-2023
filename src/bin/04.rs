advent_of_code::solution!(4);

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

struct Card {
    id: u32,
    wins: Vec<u32>,
    hand: Vec<u32>,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, id) = preceded(
            tag("Card"),
            preceded(multispace0, terminated(digit1, tag(": "))),
        )(input)?;

        let (remaining, (wins, hand)) = separated_pair(
            separated_list0(tag(" "), preceded(multispace0, digit1)),
            tag(" | "),
            separated_list0(tag(" "), preceded(multispace0, digit1)),
        )(remaining)?;

        let id = id.parse::<u32>().unwrap();
        let wins = wins
            .into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let hand = hand
            .into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Ok((remaining, Card { id, wins, hand }))
    }

    fn result(&self) -> u32 {
        self.hand.iter().filter(|n| self.wins.contains(n)).count() as _
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|l| Card::parse(l).unwrap().1)
        .collect::<Vec<_>>();

    let sum = cards
        .into_iter()
        .map(|c| {
            if c.result() == 0 {
                0
            } else {
                1 << (c.result() - 1)
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let cards = input
        .lines()
        .map(|l| Card::parse(l).unwrap().1)
        .collect::<Vec<_>>();

    cards.into_iter().for_each(|c| {
        let n = *map.entry(c.id).or_insert(1);
        for i in (&c.id + 1)..=(c.id + c.result()) {
            *map.entry(i).or_insert(1) += n;
        }
    });

    Some(map.into_values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(30));
    }
}
