fn main() {
    let x = input(String::from("Enter any character: "));
    println!("The value entered is {}",x); }

fn input(s: String) -> String {
    use std::io;
    use std::io::Write;
    print!("{}",s);
    io::stdout().flush().unwrap();
    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();
    data }
