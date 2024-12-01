use std::fs::read_to_string;

fn distance(a: u64, b: u64) -> u64 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => b - a,
        std::cmp::Ordering::Greater => a - b,
        std::cmp::Ordering::Equal => 0,
    }
}
fn main() {
    let mut left_list: Vec<u64> = Vec::new();
    let mut right_list: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for line in read_to_string("../input.txt").unwrap().lines() {
        for (index, part) in line.split_whitespace().enumerate() {
            match index {
                0 => left_list.push(part.to_string().parse().unwrap()),
                1 => right_list.push(part.to_string().parse().unwrap()),
                _ => break,
            }
        }
    }

    left_list.sort();
    right_list.sort();

    let it = left_list.iter().zip(right_list.iter());

    for (left, right) in it {
        distances.push(distance(*left, *right));
    }

    let res: u64 = distances.iter().sum();
    print!("{}", res)
}
