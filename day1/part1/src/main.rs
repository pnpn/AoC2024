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

    let dist: Vec<u64> = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(elem_left, elem_right)| distance(*elem_left, *elem_right))
        .collect();

    let res: u64 = dist.iter().sum();
    print!("{}", res)
}
