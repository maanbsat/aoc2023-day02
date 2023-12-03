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
    id: usize,
    sets: Vec<ColorCombo>,
}
struct ColorCombo {
    red: usize,
    green: usize,
    blue: usize,
}

fn combo_is_valid(combo: &ColorCombo) -> bool {
    combo.red <= 12 && combo.green <= 13 && combo.blue <= 14
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

fn main() -> Result<()> {
    let res: usize = io::BufReader::new(File::open(PATH)?)
        .lines()
        .map(|l| parse_game(&l.unwrap()).unwrap())
        .filter(|g| g.sets.iter().all(|c| combo_is_valid(c)))
        .map(|g| g.id)
        .sum();

    println!("{}", res);
    Ok(())
}
