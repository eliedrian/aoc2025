use std::env;
use std::fs;

fn max(s: &str) -> (usize, u32) {
    s.chars().enumerate().fold((0, 0), |acc, (i, digit)| {
        let d: u32 = digit.to_digit(10).unwrap();
        let (_, x) = acc;
        if x >= d { acc } else { (i, d) }
    })
}

fn compute_max_joltage(bank: &str) -> u32 {
    // find max of the first n - 1 characters
    let length = bank.len();
    let first_m_characters = &bank[..length - 1];
    let (index, first_digit) = max(first_m_characters);

    let tail = &bank[index + 1..];
    let (_, second_digit) = max(tail);

    (first_digit * 10) + second_digit
}

fn solution(input: &str) -> u32 {
    let banks: Vec<&str> = input.split_terminator('\n').collect();
    let max_joltages = banks.into_iter()
        .filter(|x| !x.is_empty())
        .map(compute_max_joltage);
    max_joltages.sum()
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Missing file.");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    let answer = solution(&file_content);
    println!("Answer: {answer}");
}
