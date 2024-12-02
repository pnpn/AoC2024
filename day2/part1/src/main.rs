use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

struct Report {
    data: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        if !self.data.is_sorted_by(|a, b| a < b) && !self.data.is_sorted_by(|a, b| a > b) {
            return false;
        }
        for pair in self.data.clone().into_iter().tuple_windows::<(i32, i32)>() {
            let diff = (pair.0 - pair.1).abs();
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    }
}

impl FromStr for Report {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            data: s
                .split_whitespace()
                .map(|elem| elem.parse().unwrap())
                .collect(),
        })
    }
}

struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    fn add(&mut self, report: Report) {
        self.reports.push(report);
    }
}

fn main() {
    let path = format!("{}/../input.txt", env!("CARGO_MANIFEST_DIR"));
    let binding = read_to_string(path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut reports = Reports {
        reports: Vec::new(),
    };
    for line in lines.iter() {
        reports.add(Report::from_str(line).unwrap());
    }

    let safe_reports = reports
        .reports
        .iter()
        .filter(|report| report.is_safe())
        .count();
    println!("{}", safe_reports)
}
