use anyhow::Result;
use itertools::Itertools;

use std::{fs, str::FromStr};

#[derive(Debug, PartialEq)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Reveal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;
        for cube_type in s.split(",").collect_vec() {
            let num_color = cube_type.trim().split(" ").collect_vec();
            match num_color.get(1).unwrap().to_owned() {
                "red" => red = num_color.get(0).unwrap().parse().unwrap(),
                "blue" => blue = num_color.get(0).unwrap().parse().unwrap(),
                "green" => green = num_color.get(0).unwrap().parse().unwrap(),
                _ => panic!("Couldn't parse {:?}", s),
            }
        }

        return Ok(Reveal { red, blue, green });
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    min_red: usize,
    min_blue: usize,
    min_green: usize,
}

impl Game {
    fn get_cube_set(&self) -> usize {
        return self.min_red * self.min_green * self.min_blue;
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //"Game <num>:"
        let colon_idx = s.find(":").unwrap();
        let id: usize = s[5..colon_idx].parse().unwrap();

        let mut min_red: usize = 0;
        let mut min_blue: usize = 0;
        let mut min_green: usize = 0;
        //"Game <num>: "
        let reveals_tmp = s[colon_idx+1..].split(";").collect_vec();
        for rev in &reveals_tmp {
            let reveal: Reveal = rev.trim().parse().unwrap();
            compare_and_set_min(&reveal.red, &mut min_red);
            compare_and_set_min(&reveal.green, &mut min_green);
            compare_and_set_min(&reveal.blue, &mut min_blue);
        }

        return Ok(Game {
            id,
            min_red,
            min_blue,
            min_green,
        });
    }
}

fn compare_and_set_min(value: &usize, min: &mut usize) {
    if value > min {
        *min = *value;
    }
}

fn main() -> Result<()> {
    let sum: usize = fs::read_to_string("./src/bin/day2.prod")?
        .lines()
        .map(|line| {
            let game: Game = line.parse().unwrap();
            return game.get_cube_set();
        })
        .sum();

    println!("{sum}");
    Ok(())
}