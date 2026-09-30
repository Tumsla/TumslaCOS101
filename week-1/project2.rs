// Project II: Sum and Average of Sales
fn main() {
    let amounts = [450_000.00, 1_500_000.00, 750_000.00, 2_850_000.00, 250_000.00];
    let sum: f64 = amounts.iter().sum();
    let average: f64 = sum / amounts.len() as f64;
    println!("Total Sales: {:.2}", sum);
    println!("Average Sales: {:.2}", average);
}
