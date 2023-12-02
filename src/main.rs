#![feature(try_blocks)]

fn main() {
    let file = std::fs::read_to_string("day1.txt").unwrap();

    let total = file.lines().map(|line| {
        // well this is horrifying
        let line = line.replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "f4r").replace("five", "f5e").replace("six", "s6x").replace("seven", "s7n").replace("eight", "e8t").replace("nine", "n9e");

        let first = line.chars().filter(|c| c.is_numeric()).next();
        let last = line.chars().rev().filter(|c| c.is_numeric()).next();

        (first, last)
    }).map(|(first, last)| {
        let x: Option<String> = try {
            let first = first?;
            let second = last?;

            format!("{}{}", first, second)
        };

        x
    })
    .filter_map(|str| str.map(|str| str.parse::<u32>()))
    .sum::<Result<u32, _>>();
    
    println!("{total:?}");
}
