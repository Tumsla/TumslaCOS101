// Practice 6: Concatenation
fn main() {
    let n1 = String::from("Pan-Atlantic");
    let n2 = String::from(" ");
    let n3 = String::from("University");

    let n4 = n1 + &n2 + &n3;

    println!("{}", n4);
}
