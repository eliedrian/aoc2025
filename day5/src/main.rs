use std::fs;
use std::env;
use std::ops::RangeInclusive;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Missing path.");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Unable to read file.");

    solution1(&file_content);
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, item: &usize) -> bool {
        (self.start..=self.end).contains(item)
    }
}

type Query = usize;

fn solution1(input: &str) {
    let parts = input.split_once("\n\n").unwrap();
    let range_strings: &Vec<&str> = &parts.0.split('\n').collect();
    let query_strings: &Vec<&str> = &parts.1.split('\n').collect();

    let ranges: Vec<Range> = range_strings.iter().map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        Range { 
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }).collect();

    let fresh_ids: Vec<Query> = query_strings.iter()
        .filter_map(|query| {
            query.parse::<usize>().ok()
        }).filter(|query| {
            ranges.iter().any(|r| r.contains(query))
        }).collect();

    println!("Fresh: {}", fresh_ids.len())
}
