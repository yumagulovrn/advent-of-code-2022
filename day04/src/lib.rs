pub fn main() {
    let ranges: Vec<(u8, u8, u8, u8)> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let (a, b) = l.split_once('-').unwrap();
            let (c, d) = r.split_once('-').unwrap();
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .collect();

    let part1 = ranges
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count();

    let part2 = ranges
        .into_iter()
        .filter(|(a, b, c, d)| {
            (a >= c && a <= d) || (b >= c && b <= d) || (a <= c && b >= c) || (a <= d && b >= d)
        })
        .count();

    println!("{}", part1);
    println!("{}", part2);
}
