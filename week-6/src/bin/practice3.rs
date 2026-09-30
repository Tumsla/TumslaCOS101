// Practice 3: replace()
fn main() {
    let name1 = String::from("Ayomide Lawal");
    let faculty = String::from("Faculty of Science");

    let name2 = name1.replace("Ayomide", "Adebare");
    let school = faculty.replace("Faculty", "School");

    println!("Original name: {}", name1);
    println!("New name: {}", name2);
    println!("Original faculty: {}", faculty);
    println!("New faculty: {}", school);
}
