// Practice 4: push_str()
fn main() {
    let mut school = String::from("School of Science");
    println!("Before: {}", school);

    school.push_str(" and Technology");

    println!("After: {}", school);
    println!("Length: {}", school.len());
}
