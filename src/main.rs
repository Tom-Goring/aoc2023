#![feature(try_blocks)]

use std::str::FromStr;
use strum_macros::EnumString;

use itertools::Itertools;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let file = std::fs::read_to_string("day2.txt").unwrap();

    let total = file
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let id = chars
                .by_ref()
                .skip_while(|c| c != &' ')
                .skip(1)
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let impossible = chars
                .skip(1)
                .collect::<String>()
                .split(";")
                .map(str::trim)
                .map(str::to_owned)
                .map(|game| {
                    game.split(',')
                        .map(str::trim)
                        .map(str::to_owned)
                        .map(|game| game.split(' ').map(str::to_owned).collect_vec())
                        .map(|array| {
                            (
                                array[0].clone().parse::<usize>().unwrap(),
                                Color::from_str(&array[1]).unwrap(),
                            )
                        })
                        .fold((usize::MAX, usize::MAX, usize::MAX), |acc, game| match game.1 {
                            Color::Red if game.0 < acc.0 =>  (game.0, acc.1, acc.2) ,
                            Color::Green if game.0 < acc.1 => (acc.0, game.0, acc.2),
                            Color::Blue if game.0 < acc.2 => (acc.0, acc.1, game.0),
                            _ => acc
                        })
                })
                .map(|acc| (if acc.0 == usize::MAX {0} else { acc.0 }, if acc.1 == usize::MAX {0} else { acc.1 }, if acc.2 == usize::MAX {0} else { acc.2 }))
                .fold((0, 0, 0), |acc, game| (acc.0.max(game.0), acc.1.max(game.1), acc.2.max(game.2)));

            (id, impossible)
        })
        .map(|(_, cubes)| cubes.0 * cubes.1 * cubes.2)
        .sum::<usize>();

    dbg!(total);
}
