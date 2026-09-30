// Practice 1: String Literals
fn main() {
    let name = "Aisha Lawal";
    let uni: &str = "Pan-Atlantic University";
    let address: &str = "Km 52, Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    let department: &'static str = "Computer Science";
    let school: &'static str = "School of Science";

    println!("Name: {}", name);
    println!("University: {}", uni);
    println!("Address: {}", address);
    println!("Department: {}", department);
    println!("School: {}", school);
}
