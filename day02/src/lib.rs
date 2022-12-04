pub fn main() {
    let rounds = include_str!("../input.txt")
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap() as i32 - 65,
                line.chars().nth(2).unwrap() as i32 - 88,
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let part1: i32 = rounds
        .clone()
        .into_iter()
        .map(|(a, b)| {
            if a == b {
                return b + 1 + 3;
            } else if (a + 1) % 3 == b {
                return b + 1 + 6;
            }
            return b + 1;
        })
        .sum();

    let part2: i32 = rounds
        .into_iter()
        .map(|(a, b)| {
            if b == 0 {
                return (a + 2) % 3 + 1;
            } else if b == 1 {
                return a + 1 + 3;
            }
            return (a + 1) % 3 + 1 + 6;
        })
        .sum();

    println!("{}", part1);
    println!("{}", part2);
}
