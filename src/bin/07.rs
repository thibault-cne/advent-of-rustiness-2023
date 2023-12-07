advent_of_code::solution!(7);

use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut hands) =
        many1(terminated(Hand::<PartOne>::parse, line_ending))(input).expect("parsing error");

    hands.sort_unstable();

    let ans = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut hands) =
        many1(terminated(Hand::<PartTwo>::parse, line_ending))(input).expect("parsing error");

    hands.sort_unstable();

    let ans = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum();

    Some(ans)
}

trait Parts {
    fn part() -> Part;
}

struct PartOne;
struct PartTwo;

impl Parts for PartOne {
    fn part() -> Part {
        Part::One
    }
}

impl Parts for PartTwo {
    fn part() -> Part {
        Part::Two
    }
}

enum Part {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum CardPart {
    One(CardOne),
    Two(CardTwo),
}

impl CardPart {
    fn parse(input: char, part: Part) -> CardPart {
        macro_rules! parse {
            ($card_part:ident, $card:ident) => {
                match input {
                    'A' => Self::$card_part($card::A),
                    'K' => Self::$card_part($card::K),
                    'Q' => Self::$card_part($card::Q),
                    'J' => Self::$card_part($card::J),
                    'T' => Self::$card_part($card::T),
                    '9' => Self::$card_part($card::Nine),
                    '8' => Self::$card_part($card::Eight),
                    '7' => Self::$card_part($card::Seven),
                    '6' => Self::$card_part($card::Six),
                    '5' => Self::$card_part($card::Five),
                    '4' => Self::$card_part($card::Four),
                    '3' => Self::$card_part($card::Three),
                    '2' => Self::$card_part($card::Two),
                    _ => unreachable!(),
                }
            };
        }

        match part {
            Part::One => parse!(One, CardOne),
            Part::Two => parse!(Two, CardTwo),
        }
    }
}

macro_rules! cards {
    ($(($ident:ident, [$($e:ident),*])),*) => {
        $(
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
            enum $ident {
                $($e),*
            }
        )*
    };
}

cards!(
    (
        CardOne,
        [Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A]
    ),
    (
        CardTwo,
        [J, Two, Three, Four, Five, Six, Seven, Eight, Nine, T, Q, K, A]
    )
);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard([CardPart; 5]),
    OnePair([CardPart; 5]),
    TwoPair([CardPart; 5]),
    ThreeOfAKind([CardPart; 5]),
    FullHouse([CardPart; 5]),
    FourOfAKind([CardPart; 5]),
    FiveOfAKind([CardPart; 5]),
}

impl HandType {
    fn parse(input: Vec<CardPart>, part: Part) -> HandType {
        match part {
            Part::One => Self::parse_one(input),
            Part::Two => Self::parse_two(input),
        }
    }

    fn parse_one(input: Vec<CardPart>) -> Self {
        let counts = input
            .iter()
            .collect::<HashSet<_>>()
            .iter()
            .map(|v| input.iter().filter(|card| card == v).count())
            .collect::<Vec<usize>>();
        let cards = [input[0], input[1], input[2], input[3], input[4]];

        match counts.len() {
            1 => Self::FiveOfAKind(cards),
            2 if counts[0] == 1 || counts[1] == 1 => Self::FourOfAKind(cards),
            2 => Self::FullHouse(cards),
            3 if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 => Self::ThreeOfAKind(cards),
            3 => Self::TwoPair(cards),
            4 => Self::OnePair(cards),
            5 => Self::HighCard(cards),
            _ => unreachable!(),
        }
    }

    fn parse_two(input: Vec<CardPart>) -> HandType {
        let items = input.iter().collect::<HashSet<_>>();
        let counts = items
            .iter()
            .map(|v| input.iter().filter(|card| card == v).count())
            .collect::<Vec<usize>>();
        let js = input
            .iter()
            .filter(|v| **v == CardPart::Two(CardTwo::J))
            .count();

        let cards = [input[0], input[1], input[2], input[3], input[4]];

        match counts.len() {
            1 => Self::FiveOfAKind(cards),
            2 if js > 0 => Self::FiveOfAKind(cards),
            2 if counts[0] == 1 || counts[1] == 1 => Self::FourOfAKind(cards),
            2 => Self::FullHouse(cards),
            3 if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 => {
                if js > 0 {
                    Self::FourOfAKind(cards)
                } else {
                    Self::ThreeOfAKind(cards)
                }
            }
            3 => match js {
                2 => Self::FourOfAKind(cards),
                1 => Self::FullHouse(cards),
                0 => Self::TwoPair(cards),
                _ => unreachable!(),
            },
            4 => match js {
                1 | 2 => Self::ThreeOfAKind(cards),
                0 => Self::OnePair(cards),
                _ => unreachable!(),
            },
            5 => match js {
                1 => Self::OnePair(cards),
                0 => Self::HighCard(cards),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Hand<T> {
    bid: u64,
    hand: HandType,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> std::cmp::PartialEq for Hand<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl<T> std::cmp::PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

impl<T> std::cmp::Eq for Hand<T> {}

impl<T> std::cmp::Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl<T: Parts> Hand<T> {
    fn parse(input: &str) -> IResult<&str, Hand<T>> {
        let (rem, (cards, bid)) =
            separated_pair(complete::alphanumeric0, tag(" "), complete::u64)(input)?;

        let hand = cards
            .chars()
            .map(|c| CardPart::parse(c, T::part()))
            .collect::<Vec<_>>();

        Ok((
            rem,
            Hand {
                bid,
                hand: HandType::parse(hand, T::part()),
                _phantom: std::marker::PhantomData,
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(5905));
    }
}
