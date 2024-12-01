use std::fs::read_to_string;

fn main() {
    let mut left_list: Vec<u64> = Vec::new();
    let mut right_list: Vec<u64> = Vec::new();

    for line in read_to_string("../input.txt").unwrap().lines() {
        for (index, part) in line.split_whitespace().enumerate() {
            match index {
                0 => left_list.push(part.to_string().parse().unwrap()),
                1 => right_list.push(part.to_string().parse().unwrap()),
                _ => break,
            }
        }
    }

    let path = format!("{}/../input.txt", env!("CARGO_MANIFEST_DIR"));
    let binding = read_to_string(path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let left: Vec<u64> = lines
        .clone()
        .iter()
        .map(|line| {
            (**line)
                .split_whitespace()
                .next()
                .unwrap()
                .to_owned()
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    let right: Vec<u64> = lines
        .clone()
        .iter()
        .map(|line| {
            (**line)
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_owned()
                .parse()
                .unwrap()
        })
        .collect();

    let occurences: Vec<u64> = left
        .iter()
        .map(|elem| *elem * (right.iter().filter(|i| **i == *elem).count()) as u64)
        .collect();

    let res: u64 = occurences.iter().sum();

    print!("{}", res)
}
