// Practice 5: trim()
fn main() {
    let fullname = String::from(" Pan-Atlantic University ");

    println!("Before trim, length: {}", fullname.len());
    println!("After trim, length: {}", fullname.trim().len());
    println!("Trimmed: {}", fullname.trim());
}
