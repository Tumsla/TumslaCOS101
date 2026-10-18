fn main() {
    // the six APS levels, in order
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // job titles for each profession, in the same order as the levels
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("APS Level Checker");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    println!("Choose your profession (1 - 4)");

    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice: i32 = input1.trim().parse().expect("Invalid input");

    println!("Enter your job title");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let title: String = input2.trim().parse().expect("Invalid input");

    let mut found = false;

    // go through the six positions and compare the title with the one at each index
    for i in 0..levels.len() {
        if choice == 1 {
            if title == office_admin[i] {
                println!("{} holds position {}", title, levels[i]);
                found = true;
            }
        } else if choice == 2 {
            if title == academic[i] {
                println!("{} holds position {}", title, levels[i]);
                found = true;
            }
        } else if choice == 3 {
            if title == lawyer[i] {
                println!("{} holds position {}", title, levels[i]);
                found = true;
            }
        } else if choice == 4 {
            if title == teacher[i] {
                println!("{} holds position {}", title, levels[i]);
                found = true;
            }
        }
    }

    if found == false {
        println!("Invalid choice or job title, please check the table and try again");
    }
}
