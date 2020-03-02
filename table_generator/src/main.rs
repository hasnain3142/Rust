use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Multiplication Table Generator");
    print!("Enter range of table: ");
    io::stdout().flush().unwrap();
    let mut r = String::new();
    io::stdin().read_line(&mut r).unwrap();
    let y: i32 = r.trim().parse().unwrap();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let x: i32 = n.trim().parse().unwrap();
    for i in 1..y+1{
        println!("{} x {} = {}",x,i,x*i);
    }
}
