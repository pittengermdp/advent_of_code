use anyhow::Result;
use nom::{
    character::complete::digit1,
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
    {
        branch::alt,
        bytes::complete::{tag, take_while},
        combinator::map,
        error::ParseError,
        sequence::{delimited, tuple},
    },
};
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Rgb {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    rounds: Vec<Rgb>,
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        nom::character::complete::space0,
        inner,
        nom::character::complete::space0,
    )
}

fn game_tag_parser(input: &str) -> IResult<&str, &str> {
    ws(tag("Game"))(input)
}

fn num_parser(input: &str) -> IResult<&str, &str> {
    take_while(char::is_numeric)(input)
}

fn colon_parser(input: &str) -> IResult<&str, &str> {
    tag(":")(input)
}

fn color_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("blue"), tag("green")))(input)
}

fn game_id_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn color_number_parser(input: &str) -> IResult<&str, (u32, &str)> {
    tuple((ws(game_id_parser), ws(color_parser)))(input)
}

fn get_color_set(input: &str) -> IResult<&str, Vec<Rgb>> {
    let set_parser = map(
        separated_list1(ws(tag(",")), color_number_parser),
        |pairs: Vec<(u32, &str)>| {
            pairs
                .into_iter()
                .fold(Rgb::default(), |mut acc, (count, color)| {
                    match color {
                        "red" => acc.red += count,
                        "green" => acc.green += count,
                        "blue" => acc.blue += count,
                        _ => (),
                    }
                    acc
                })
        },
    );

    separated_list0(ws(tag(";")), set_parser)(input)
}

fn newline_parser(input: &str) -> IResult<&str, &str> {
    let (remaining, parsed) = alt((tag("\r\n"), tag("\n")))(input)?;
    Ok((remaining, parsed))
}

fn games_parser(input: &str) -> Result<Vec<Game>> {
    let game_parser = map(
        tuple((
            ws(game_tag_parser),
            ws(game_id_parser),
            ws(colon_parser),
            get_color_set,
        )),
        |(_, id, _, rounds)| Game { id, rounds },
    );

    Ok(separated_list0(newline_parser, game_parser)(input)?.1)
}

#[aoc(day2, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let input = games_parser(input).unwrap();
    let max_cubes = Rgb {
        red: 12,
        green: 12,
        blue: 14,
    };
    input
        .iter()
        .filter(|&game| game.rounds.iter().all(|rgb| rgb <= &max_cubes))
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
#[must_use]
pub fn part2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_tag_parser() {
        let input = "Game 1: ";
        assert_eq!(game_tag_parser(input), Ok(("1: ", "Game")));
    }

    #[test]
    fn test_num_parser() {
        let input = "123abc";
        assert_eq!(num_parser(input), Ok(("abc", "123")));
    }

    #[test]
    fn test_color_parser() {
        let input = "red";
        assert_eq!(color_parser(input), Ok(("", "red")));
    }

    #[test]
    fn test_color_number_parser() {
        let input = "3 blue,";
        assert_eq!(color_number_parser(input), Ok((",", (3, "blue"))));
    }

    #[test]
    fn test_get_color_set() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = vec![
            Rgb {
                red: 4,
                green: 0,
                blue: 3,
            },
            Rgb {
                red: 1,
                green: 2,
                blue: 6,
            },
            Rgb {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];
        assert_eq!(get_color_set(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_games() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected = vec![
            Game {
                id: 1,
                rounds: vec![
                    Rgb {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Rgb {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Rgb {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Rgb {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Rgb {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Rgb {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Rgb {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Rgb {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Rgb {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Rgb {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Rgb {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Rgb {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Rgb {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Rgb {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];

        assert_eq!(games_parser(&input).unwrap(), expected);
    }
}
