use std::io;

fn main()
{
    let mut input1 = String::new();

    println!("Enter a");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();

    println!("Enter b");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Not a valid number");


    let mut input3 = String::new();
    println!("Enter c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = b*b - 4.0*a*c;

    if d >= 0.0
    {
        let root1 = (-b + d.sqrt()) / 2.0*a;
        let root2 = (-b - d.sqrt()) / 2.0*a;
    println!("{}", root1); println!("{}", root2);
    println!("Two distinct roots!")
    } 
    else if d == 0.0
    {
        let root1 = (-b + d.sqrt()) / 2.0*a;
        let root2 = (-b - d.sqrt()) / 2.0*a;
    println!("{}", root1); println!("{}", root2);
    println!("Exactly one real root!")
    } 
    else if d <= 0.0
    {
        let root1 = (-b + d.sqrt()) / 2.0*a;
        let root2 = (-b - d.sqrt()) / 2.0*a;
    println!("{}", root1); println!("{}", root2);
    println!("No real roots!")
    }
    

        
    
}
