use itertools::Itertools;

pub fn main() {
    let data = include_bytes!("../input.txt");
    let (state_data, moves_data) =
        data.split_at(data.windows(2).position(|b| b == b"\n\n").unwrap() + 2);
    let (mut states, mut swaps): ([Vec<u8>; 9], _) = (Default::default(), [0; 64]);

    state_data
        .split(|b| b == &b'\n')
        .rev()
        .skip(2)
        .for_each(|line| {
            line.iter()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c != &&b' ')
                .for_each(|(i, c)| states[i].push(*c))
        });

    let mut states2 = states.clone();

    moves_data.split(|b| b == &b'\n').for_each(|mov| {
        let (n, a, b): (usize, _, _) = mov
            .split(|b| b == &b' ')
            .skip(1)
            .step_by(2)
            .map(|n| atoi::atoi(n).unwrap())
            .collect_tuple()
            .unwrap();

        for _ in 0..n {
            let tmp = states[a - 1].pop().unwrap();
            states[b - 1].push(tmp);
        }

        let len = states2[a - 1].len();
        let swap = &mut swaps[..n];

        swap.copy_from_slice(&states2[a - 1][len - n..len]);
        states2[a - 1].truncate(len - n);
        states2[b - 1].extend(swap.iter());
    });

    let part1: String = states.iter().map(|s| *s.last().unwrap() as char).collect();
    let part2: String = states2.iter().map(|s| *s.last().unwrap() as char).collect();

    println!("{}", part1);
    println!("{}", part2);
}
