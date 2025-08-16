use std::io::{self,Write};
use std::process;

fn add(a: f32,b: f32) -> f32 {a+b}
fn sub(a: f32,b: f32) -> f32 {a-b}
fn mul(a: f32,b: f32) -> f32 {a*b}
fn div(a: f32,b: f32) -> f32 {
    if b == 0 {println!("Error: Division by 0!"); process::exit(1);}
    else {a/b}
}

fn main() {
    let mut choise = String::new();
    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("1. Add");
    println!("2. Sub");
    println!("3. Mul");
    println!("4. Div");

    print!("\nPick one: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choise).expect("Input failure");
    let option: i32 = choise.trim().parse().expect("Invalid number");

    if option < 1 || option > 4 {
        println!("Invalid choise");
        process::exit(1);
    }

    print!("Enter 1st number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number1).expect("Error: Input failure");
    let num1: f32 = number1.trim().parse().expect("Error: Invalid input");

    print!("Enter 2nd number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number2).expect("Error: Input failure");
    let num2: f32 = number2.trim().parse().expect("Error: Invalid input");

    match option {
        1 => println!("{} + {} = {}",num1,num2,add(num1,num2)),
        2 => println!("{} - {} = {}",num1,num2,sub(num1,num2)),
        3 => println!("{} * {} = {}",num1,num2,mul(num1,num2)),
        4 => println!("{} / {} = {}",num1,num2,div(num1,num2)),
        _ => println!("Something went wrong"),
    }
}
