advent_of_code::solution!(2);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
struct Game {
    pub id: u32,
    pub reveals: Vec<Vec<Cube>>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, game_id) = preceded(tag("Game "), terminated(digit1, tag(": ")))(input)?;

        let cube_parser = separated_list0(tag(", "), Cube::parse);
        let (remaining, reveals) = separated_list0(tag("; "), cube_parser)(remaining)?;

        Ok((
            remaining,
            Game {
                id: game_id.parse().unwrap(),
                reveals,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Cube {
    Green(u32),
    Blue(u32),
    Red(u32),
}

impl Cube {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, values) = separated_pair(
            digit1,
            complete::char(' '),
            alt((tag("blue"), tag("red"), tag("green"))),
        )(input)?;

        match values.1 {
            "blue" => Ok((remaining, Cube::Blue(values.0.parse().unwrap()))),
            "red" => Ok((remaining, Cube::Red(values.0.parse().unwrap()))),
            "green" => Ok((remaining, Cube::Green(values.0.parse().unwrap()))),
            _ => unreachable!(),
        }
    }

    fn compare(&self, other: &Self) -> bool {
        match (self, other) {
            (Cube::Green(l), Cube::Green(r)) => l > r,
            (Cube::Red(l), Cube::Red(r)) => l > r,
            (Cube::Blue(l), Cube::Blue(r)) => l > r,
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    let lines = input.split('\n').collect::<Vec<&str>>();

    for l in lines {
        let game = Game::parse(l).unwrap().1;

        let closure = |d: &Vec<Cube>| {
            !d.iter().any(|c| {
                c.compare(&Cube::Red(12))
                    || c.compare(&Cube::Green(13))
                    || c.compare(&Cube::Blue(14))
            })
        };

        if game.reveals.iter().all(closure) {
            ans += game.id;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    let lines = input.split('\n').collect::<Vec<&str>>();

    for l in lines {
        let game = Game::parse(l).unwrap().1;

        let closure = |d: &Vec<Cube>| {
            d.iter().fold((0, 0, 0), |acc, c| match *c {
                Cube::Green(g) if acc.0 < g => (g, acc.1, acc.2),
                Cube::Blue(b) if acc.1 < b => (acc.0, b, acc.2),
                Cube::Red(r) if acc.2 < r => (acc.0, acc.1, r),
                _ => acc,
            })
        };

        let max = game
            .reveals
            .iter()
            .map(closure)
            .fold((0, 0, 0), |acc, val| {
                (acc.0.max(val.0), acc.1.max(val.1), acc.2.max(val.2))
            });
        ans += max.0 * max.1 * max.2;
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn parse_cube() {
        assert_eq!(Cube::parse("1 blue").unwrap().1, Cube::Blue(1));
        assert_eq!(Cube::parse("12 green").unwrap().1, Cube::Green(12));
        assert_eq!(Cube::parse("21 red").unwrap().1, Cube::Red(21));
        assert_eq!(Cube::parse("100 blue").unwrap().1, Cube::Blue(100));
    }

    #[test]
    fn parse_game() {
        let vec = vec![
            vec![Cube::Blue(1), Cube::Green(2), Cube::Red(3)],
            vec![Cube::Blue(4), Cube::Green(5), Cube::Red(6)],
            vec![Cube::Blue(7), Cube::Green(8), Cube::Red(9)],
        ];
        let game = Game {
            id: 1,
            reveals: vec.clone(),
        };
        assert_eq!(
            Game::parse(
                "Game 1: 1 blue, 2 green, 3 red; 4 blue, 5 green, 6 red; 7 blue, 8 green, 9 red;"
            )
            .unwrap()
            .1,
            game
        );
    }
}
