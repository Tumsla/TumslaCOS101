// Project III: Depreciation
fn main() {
    let p: f64 = 210_000.0;
    let r: f64 = 5.0;
    let n: f64 = 3.0;
    let a = p * (1.0 - (r / 100.0)).powf(n);
    println!("Value of TV after {} years is {:.2}", n, a);
}
