fn main() {
    // each candidate is a tuple of (name, years of experience)
    let mut people: Vec<(String, i32)> = Vec::new();

    let mut input1 = String::new();
    println!("How many candidates are you interviewing?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num: i32 = input1.trim().parse().expect("Invalid input");

    // push each candidate into the vector
    for count in 0..num {
        let mut input2 = String::new();
        println!("Enter name of candidate {}", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let name: String = input2.trim().parse().expect("Invalid input");

        let mut input3 = String::new();
        println!("Enter years of programming experience for {}", name);
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
        let years: i32 = input3.trim().parse().expect("Invalid input");

        people.push((name, years));
    }

    // best holds the index of the person with the most years so far
    let mut best = 0;

    for i in 0..people.len() {
        if people[i].1 > people[best].1 {
            best = i;
        }
    }

    println!("The candidate with the highest experience is {}", people[best].0);
    println!("{} has {} years of programming experience", people[best].0, people[best].1);
}

