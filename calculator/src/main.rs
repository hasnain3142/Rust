use std::io;
use std::io::Write;
fn main() {
    println!("Welcome to My Calculator");
    loop {
    println!("Select any operation:\n1.Addition\n2.Subtraction\n3.Multiplication\n4.Division");
    print!("Enter operation number: ");
    io::stdout().flush().unwrap();
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    let s = s1.trim();
    let mut i1= String::new();
    print!("Enter first operand: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut i1).unwrap();
    let x: f64 = i1.trim().parse().unwrap();
    let mut i2 = String::new();
    print!("Enter second operand: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut i2).unwrap();
    let y: f64 = i2.trim().parse().unwrap();

    if s=="1" {
        println!("Result: {}",x+y);
    }
    else if s=="2" {
        println!("Result: {}",x-y);
    }
    else if s =="3" {
        println!("Result: {}",x*y);
    }
    else if s=="4" {
        println!("Result: {}",x/y)
    }
    else {
        println!("Please select correct operation")
    }
    print!("Press 'Y' to continue using My Calculator: ");
    io::stdout().flush().unwrap();
    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let choice: char = c.trim().parse().unwrap();
    if choice != 'Y' {
        break
        }
    }
}

