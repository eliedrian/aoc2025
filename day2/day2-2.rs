use std::env;
use std::fs;

fn digit_count(num: &u64) -> u32 {
    match num {
        0 => 0,
        _ => num.ilog10() + 1
    }
}

fn invalid_id(id: &u64) -> bool {
    let length: u32 = digit_count(id);

    let mut invalid = false;

    for i in 2..=length {
        invalid = true;
        let slice_size = length / i;
        let mut x: u64 = *id;
        let current_pattern = x.rem_euclid(10u64.pow(slice_size));
        while x > 0 {
            let slice = x.rem_euclid(10u64.pow(slice_size));

            if digit_count(&slice) != slice_size {
                invalid = false;
                break;
            }

            x = x / 10u64.pow(slice_size);

            if current_pattern != slice {
                invalid = false;
                break;
            }
        }

        // id is still invalid after a pass, must mean it has repeating digits
        if invalid {
            break;
        }
        
        // checked smallest slice size, break early
        if slice_size == 1 {
            break;
        }
    }

    if invalid {
        println!("invalid: {id}");
    }
    invalid
}

fn select_invalid_ids(range: Option<(&str, &str)>) -> Vec<String> {
    match range {
        Some((start, end)) => {
            let s: u64 = start.parse().unwrap();
            let e: u64 = end.trim().parse().unwrap();
            (s..=e).filter(invalid_id)
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
