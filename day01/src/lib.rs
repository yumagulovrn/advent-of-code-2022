use itertools::Itertools;

pub fn main() {
    let calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .sorted_unstable()
        .rev()
        .take(3)
        .collect::<Vec<u32>>();

    println!("{}", calories[0]);
    println!("{}", calories.into_iter().sum::<u32>())
}
