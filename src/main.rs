#![feature(try_blocks)]

use strum_macros::EnumString;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum Color {
    Green,
    Blue,
    Red
}

fn main() {
    let file = std::fs::read_to_string("day2.txt").unwrap();

    let total = file.lines().map(|line| {
        let mut chars = line.chars();

        let id = chars.by_ref().skip_while(|c| c != &' ').skip(1).take_while(|c| c.is_numeric()).collect::<String>().parse::<usize>().unwrap();

        let impossible = chars
            .skip(1)
            .collect::<String>()
            .split(";")
            .map(str::trim)
            .map(str::to_owned)
            .map(|game| game.split(',').map(str::trim).map(str::to_owned).map(|game| game.split(' ').map(str::to_owned).collect_vec()).map(|array| (array[0].clone().parse::<usize>().unwrap(), Color::from_str(&array[1]).unwrap())).find(|game| match game.1 {
                Color::Green => game.0 > 13,
                Color::Blue => game.0 > 14,
                Color::Red => game.0 > 12,
            })).any(|game| game.is_some());

        // dbg!(&id, &impossible);

        (id, impossible)
    }).filter(|(_, impossible)| !*impossible).map(|(id, _)| id).sum::<usize>();

    dbg!(total);
}
