use std::io::{self,Write};
use std::process;

fn add(a: f32,b: f32) -> f32 {a+b}
fn sub(a: f32,b: f32) -> f32 {a-b}
fn mul(a: f32,b: f32) -> f32 {a*b}
fn div(a: f32,b: f32) -> f32 {
    if b == 0.0 {
        println!("Error: Division by 0!");
        process::exit(1);
    }
    else {a/b}
}

fn read_input(prompt: &str) -> f32 {
    let mut input = String::new();

    print!("{}",prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid input");
            process::exit(1);
        }
    }
}

fn main() {
    let mut choise = String::new();

    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    print!("\nPick one: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choise).unwrap();
    
    let option: i32 = match choise.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid input");
            process::exit(1);
        }
    };

    if option < 1 || option > 4 {
        println!("Error: Invalid choise");
        process::exit(1);
    }

    let num1 = read_input("Enter the 1st number: ");
    let num2 = read_input("Enter the 2nd number: ");

    match option {
        1 => println!("{} + {} = {}",num1,num2,add(num1,num2)),
        2 => println!("{} - {} = {}",num1,num2,sub(num1,num2)),
        3 => println!("{} * {} = {}",num1,num2,mul(num1,num2)),
        4 => println!("{} / {} = {}",num1,num2,div(num1,num2)),
        _ => println!("Something went wrong"),
    }
}
