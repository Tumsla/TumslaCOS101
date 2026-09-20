use std::io;

fn main() {
    // Read years of experience
    let mut experience_input = String::new();
    println!("Enter years of experience:");
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read input");
    let experience: u32 = experience_input
        .trim()
        .parse()
        .expect("Input not an integer");

    // Read age
    let mut age_input = String::new();
    println!("Enter age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Input not an integer");

    // An employee is considered experienced with 2 or more years on the job
    let is_experienced = experience >= 2;

    let incentive: u32 = if is_experienced && age >= 40 {
        1_560_000
    } else if is_experienced && age >= 30 && age <= 39 {
        1_480_000
    } else if is_experienced && age < 28 {
        1_300_000
    } else if is_experienced {
        // Covers the 28-29 gap left open by the original bands
        1_300_000
    } else {
        100_000
    };

    println!("Annual incentive: N{}", incentive);
}


