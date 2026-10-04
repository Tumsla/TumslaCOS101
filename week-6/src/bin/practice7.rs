// Practice 7: The format! Macro
fn main() {
    let k1 = String::from("C");
    let k2 = String::from("S");
    let k3 = String::from("C");
    let k4 = String::from("1");
    let k5 = String::from("0");
    let k6 = String::from("1");

    let k7 = format!("{} {} {} {} {} {}", k1, k2, k3, k4, k5, k6);

    println!("\n {}", k7);
}
