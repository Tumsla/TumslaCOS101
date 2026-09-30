// Practice 2: Creating a String Object
fn main() {
    let empty = String::new();
    let course = String::from("ComputerScience");

    println!("Empty String: \"{}\", length {}", empty, empty.len());
    println!("Course: {}, length {}", course, course.len());
}
