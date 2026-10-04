// Project: The Restaurant Menu
use std::io;

fn main() {
    println!("===== MENU =====");
    println!("P  Poundo Yam / Edinkaiko Soup  N3200");
    println!("F  Fried Rice & Chicken         N3000");
    println!("A  Amala & Ewedu Soup           N2500");
    println!("E  Eba & Egusi Soup             N2000");
    println!("W  White Rice & Stew            N2500");

    println!("Enter the food type:");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read line");
    let food = food.trim();

    println!("Enter the quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read line");
    let qty: u32 = qty.trim().parse().expect("Please enter a number");

    let price: u32;
    if food == "P" {
        price = 3200;
    } else if food == "F" {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "E" {
        price = 2000;
    } else if food == "W" {
        price = 2500;
    } else {
        println!("Invalid food type");
        return;
    }

    let mut total = price * qty;
    println!("Total: N{}", total);

    if total > 10000 {
        total = total - total * 5 / 100;
        println!("5% discount applied. New total: N{}", total);
    }
}
