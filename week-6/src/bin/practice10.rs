// Practice 10: Logical
fn main() {
    let a = 10;
    let b = 20;
    let c = 10;
    let d = 20;
    let is_elder = false;

    println!("{}", (a > 10) && (b > 10));
    println!("{}", (c > 10) || (d > 10));

    if !is_elder {
        println!("Not Elder");
    }
}
