fn get_marker(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, chars)| !(1..size).any(|i| chars[..i].contains(&chars[i])))
        .unwrap()
        .0
        + size
}

pub fn main() {
    let data = include_str!("../input.txt").trim_end();

    let part1 = get_marker(data, 4);
    let part2 = get_marker(data, 14);

    println!("{}", part1);
    println!("{}", part2);
}
