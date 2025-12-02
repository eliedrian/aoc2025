use std::env;
use std::fs;

fn valid_id(id: &u64) -> bool {
    let length: u32 = id.ilog10() + 1;
    let midpoint: u32 = length / 2;
    let slice_a: u64 = id / 10u64.pow(midpoint);
    let slice_b: u64 = id.rem_euclid(10u64.pow(midpoint));

    slice_a == slice_b
}

fn select_invalid_ids(range: Option<(&str, &str)>) -> Vec<String> {
    match range {
        Some((start, end)) => {
            let s: u64 = start.parse().unwrap();
            let e: u64 = end.trim().parse().unwrap();
            (s..e).filter(valid_id)
                .map(|id| id.to_string()).collect::<Vec<String>>()
        },
        _ => vec![]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Missing file path.");
        std::process::exit(1);
    }
    let file_path: &str = &args[1];
    let file_content: String = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    let ranges = file_content.split(",");
    let invalid_ids = ranges.into_iter()
        .map(|range| range.split_once("-"))
        .flat_map(select_invalid_ids);
    let answer = invalid_ids.fold(0, |acc, id| acc + id.parse::<u64>().unwrap());
    println!("answer: {answer}");
}
