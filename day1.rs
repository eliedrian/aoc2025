use std::fs;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Missing file path.");
        return ExitCode::from(1);
    }

    let file_path = &args[1];

    let read_result = fs::read_to_string(file_path);
    let content = match read_result {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return ExitCode::from(1)
        }
    };

    let mut current_safe_code = 50;
    let mut password = 0;
    let mut passes = 0;
    for line in content.lines() {
        let mut line_chars = line.chars();
        let direction = line_chars.nth(0).unwrap();

        let count = &line[1..].parse::<i32>().unwrap();

        let signed_count = match direction {
            'R' => count,
            'L' => &-count,
            _ => todo!()
        };

        let mut hits = 0; //count / 100;

        let intermediate_code = current_safe_code + signed_count;

        if current_safe_code == 0 && *signed_count < 0 {
            passes -= 1;
        }


        passes += (intermediate_code.div_euclid(100)).abs();


        current_safe_code = intermediate_code.rem_euclid(100);

        println!("add pass; {} {} now: {}", intermediate_code,  (intermediate_code.div_euclid(100)).abs(), current_safe_code);

        if current_safe_code == 0 { // && (current_safe_code <= 0 || current_safe_code >= 100) {
            hits += 1
        }

        password += hits;
    }

    println!("Password: {}", password);
    println!("Passes: {}", passes);

    ExitCode::SUCCESS
}
