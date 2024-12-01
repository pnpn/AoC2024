use std::fs::read_to_string;

fn main() {
    let mut left_list: Vec<u64> = Vec::new();
    let mut right_list: Vec<u64> = Vec::new();
    let mut occurences: Vec<u64> = Vec::new();

    for line in read_to_string("../input.txt").unwrap().lines() {
        for (index, part) in line.split_whitespace().enumerate() {
            match index {
                0 => left_list.push(part.to_string().parse().unwrap()),
                1 => right_list.push(part.to_string().parse().unwrap()),
                _ => break,
            }
        }
    }

    for num in left_list.iter() {
        let occurence = right_list.iter().filter(|i| **i == *num).count();
        occurences.push(num * (occurence as u64));
    }

    let res: u64 = occurences.iter().sum();

    print!("{}", res)
}
