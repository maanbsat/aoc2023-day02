use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

const PATH: &str = "input.txt";
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
}

struct Game {
    #[allow(dead_code)]
    id: usize,
    sets: Vec<ColorCombo>,
}

struct ColorCombo {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_set(set: &str) -> Result<ColorCombo> {
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;

    for color_record in set.split(", ") {
        let (num, color) = color_record
            .split_once(" ")
            .context(format!("Can't split color record: {}", color_record))?;
        let n: usize = num.parse()?;
        match color {
            "red" => red += n,
            "green" => green += n,
            "blue" => blue += n,
            _ => bail!("Unknown color: {}", color),
        }
    }

    Ok(ColorCombo { red, green, blue })
}

fn parse_game(line: &str) -> Result<Game> {
    let res = LINE_RE
        .captures(line)
        .context(format!("Cannot parse row: {}", line))?;
    let sets: Vec<ColorCombo> = res[2].split("; ").map(|x| parse_set(x).unwrap()).collect();

    Ok(Game {
        id: res[1].parse()?,
        sets,
    })
}

fn game_power(game: Game) -> Result<usize> {
    let minimum = game
        .sets
        .into_iter()
        .reduce(|acc, s| ColorCombo {
            red: max(acc.red, s.red),
            green: max(acc.green, s.green),
            blue: max(acc.blue, s.blue),
        })
        .context("Can't get fewest number of required cubes")?;

    Ok(minimum.red * minimum.green * minimum.blue)
}

fn main() -> Result<()> {
    let res: usize = io::BufReader::new(File::open(PATH)?)
        .lines()
        .map(|l| parse_game(&l.unwrap()).unwrap())
        .map(|g| game_power(g).unwrap())
        .sum();

    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(
            super::game_power(
                super::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                    .unwrap()
            )
            .unwrap(),
            48
        );
        assert_eq!(
            super::game_power(
                super::parse_game(
                    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                )
                .unwrap()
            )
            .unwrap(),
            12
        );
        assert_eq!(
            super::game_power(
                super::parse_game(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                )
                .unwrap()
            )
            .unwrap(),
            1560
        );
        assert_eq!(
            super::game_power(
                super::parse_game(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                )
                .unwrap()
            )
            .unwrap(),
            630
        );
        assert_eq!(
            super::game_power(
                super::parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
                    .unwrap()
            )
            .unwrap(),
            36
        );
    }
}
