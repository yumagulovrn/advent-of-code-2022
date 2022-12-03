use std::collections::HashSet;

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

pub fn main() {
    let rucksacks: Vec<&str> = include_str!("../input.txt")
        .split("\n")
        .filter(|line| line.len() > 0)
        .collect();

    let part1: u32 = rucksacks
        .iter()
        .map(|line| {
            line[..line.len() / 2]
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&line[line.len() / 2..].chars().collect::<HashSet<char>>())
                .into_iter()
                .next()
                .unwrap()
                .clone()
        })
        .map(score)
        .sum();

    let part2: u32 = rucksacks
        .chunks(3)
        .into_iter()
        .map(|group| {
            group[0]
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&group[1].chars().collect::<HashSet<char>>())
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&group[2].chars().collect::<HashSet<char>>())
                .cloned()
                .collect::<HashSet<char>>()
                .into_iter()
                .next()
                .unwrap()
        })
        .map(score)
        .sum();

    println!("{}", part1);
    println!("{}", part2);
}
