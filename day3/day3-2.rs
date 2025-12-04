use std::env;
use std::fs;
use std::convert::TryInto;
use std::ops::Add;

fn max(s: &str) -> (usize, u32) {
    s.chars().enumerate().fold((0, 0), |acc, (i, digit)| {
        let d: u32 = digit.to_digit(10).unwrap();
        let (_, x) = acc;
        if x >= d { acc } else { (i, d) }
    })
}

fn vec_to_number(digits: Vec<u32>) -> u64 {
    let length = digits.len();
    digits.into_iter().enumerate().fold(0_u64, |acc: u64, (i, digit)| {
        let y: u32 = (length - i - 1).try_into().unwrap();
        let d: u64 = u64::from(digit);
        acc.add(10_u64.pow(y) * d)
    })
}

fn compute_max_joltage(bank: &str) -> u64 {
    let length = bank.len();
    let mut needed_digits: usize = 12;

    let mut digits: Vec<u32> = vec![];

    let mut tail: &str = bank;
    let mut index: usize = 0;
    let mut sub_index: usize;
    let mut digit: u32;
    while !tail.is_empty() && needed_digits > 0 {
        tail = &bank[index..=length - needed_digits];
        needed_digits -= 1;
        (sub_index, digit) = max(tail);
        index += sub_index + 1;
        digits.push(digit);
    }

    vec_to_number(digits)
}

fn solution(input: &str) -> u64 {
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
