use std::io::{self, Write};
use std::process;

/// Read one trimmed line from the keyboard after showing a prompt.
fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    let bytes = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 0 bytes means the input stream was closed (EOF)
    if bytes == 0 {
        eprintln!("\nNo input received. Exiting.");
        process::exit(1);
    }

    input.trim().to_lowercase()
}

/// Keep asking until the user answers yes or no.
fn read_yes_no(prompt: &str) -> bool {
    loop {
        match read_line(prompt).as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer yes or no."),
        }
    }
}

/// Keep asking until the user enters a valid age (a whole number, 0 or more).
fn read_age(prompt: &str) -> u32 {
    loop {
        match read_line(prompt).parse::<u32>() {
            Ok(age) => return age,
            Err(_) => println!("Please enter a valid age as a whole number."),
        }
    }
}

/// Work out the annual incentive in naira.
/// Returns None if the criteria don't cover the employee.
fn annual_incentive(experienced: bool, age: u32) -> Option<u64> {
    if !experienced {
        return Some(100_000);
    }

    match age {
        40.. => Some(1_560_000),
        30..=39 => Some(1_480_000),
        0..=27 => Some(1_300_000),
        _ => None, // experienced employees aged 28 or 29
    }
}

/// Format a number with commas, e.g. 1560000 -> "1,560,000".
fn with_commas(amount: u64) -> String {
    let digits = amount.to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

fn main() {
    println!("Annual Incentive Calculator\n");

    let experienced = read_yes_no("Is the employee experienced? (yes/no): ");
    let age = read_age("Enter the employee's age: ");

    println!();
    match annual_incentive(experienced, age) {
        Some(amount) => println!("Annual incentive: ₦{}", with_commas(amount)),
        None => println!(
            "No incentive is defined for an experienced employee aged {}.",
            age
        ),
    }
}

