// #[derive(Debug)]
// enum Place {
//     Online,
//     Onsite,
// }
// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u32,
//     venue: Place,
// }
// fn main() 
// {
//     let hasnain = Student {
//         name: String::from("Hasnain Ali"),
//         age: 17,
//         venue: Place::Onsite,

//     };
//     let candidate = Place::Online;
//     println!("The candidate is {:#?}",candidate);
//     println!("{:#?}",hasnain);

// }

// #[derive(Debug)]
// enum Gender {
//     Male(String),
//     Female(char),
//     Other(u32)
// }
// fn main() {
//     let person1 = Gender::Male(String::from("Hasnain"));
//     let person2 = Gender::Female('F');
//     let person3 = Gender::Other(123);
        
//         println!("{:#?}",person1);
//         println!("{:?}",person2);
//         println!("{:#?}",person3);
// }

// fn main() {
//     let msg1 = Message::Quit;
//     let msg2 = Message::Write(String::from("Heyy Beautiful"));
//     let msg3 = Message::Move{x:12,y:14};
//     let msg4 = Message::ChangeColour(0,1,0);

//     println!("{:?}",msg1);
//     println!("{:?}",msg2);
//     println!("{:?}",msg3);
//     println!("{:?}",msg4);
// }
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Write(String),
//     Move{x:u32, y:u32},
//     ChangeColour(u32,u32,u32)
// }
// #[derive(Debug)]
// struct QuitMessage;
// #[derive(Debug)]
// struct WriteMessage(String);
// fn main() {
//     let msg1 = QuitMessage;
//     let msg2 = WriteMessage(String::from("Hello, Hasnain"));
//     println!("{:?}",msg1);
//     println!("{:?}",msg2);
// }

// fn main() {
//     let msg1 = Message::Write(String::from("Hey Hasnain"));
//     let msg2 = Message::Read(String::from("Looking Good"));
//     let msg3 = Message::Learn(14);
//     msg1.call();
//     msg2.call();
//     msg3.call()
// }
// #[derive(Debug)]
// enum Message{
//     Write(String),
//     Read(String),
//     Learn(u32)
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}",self);
//     }
// }

// #[derive(Debug)]
// enum Candidate{
  
//     Undergraduate,
//     Graduate
// }
// fn main() {
//     let ali = Candidate::Graduate;
//     let hasnain = Candidate::Undergraduate;
//     interview(ali);
//     interview(hasnain);
// 
// fn interview(x:Candidate) {
//     println!("The Candidate Is {:?}",x);
// }
// // #[derive(Debug)]
// // enum Option <T> {
// //     Some(T),
// //     None
// // }
// fn main() {
//     let number = Option::Some(14);
//     let name = Option::Some(String::from("Hasnain"));
//     let word = Option::Some("Infallible");
//     let tuple = Option::Some((1,2.0,3,'A'));
//     let null: Option<i32> = None;
    
//     println!("{:?}",number);
//     println!("{:?}",name);
//     println!("{:?}",word);
//     println!("{:?}",tuple);
//     println!("{:?}",null);

// }
// 
                                                                                                            //Match
// fn main() {
//     let number = 3;
//     match number {
//         1 => println!("Number is {}",number),
//         2 => println!("Number is {}",number),
//         3 => println!("Number is {}",number),
//         4 => println!("Number is {}",number),
//         5 => println!("Number is {}",number),
//         _ => println!("None"),
//     }
// }                                                                                                            

// fn main() {
//     let letter = 'A';
//     match letter {
//         'c' => println!("letter is {}",letter),
//         'h' => println!("letter is {}",letter),
//         'b' => println!("letter is {}",letter),
//         'A'=> println!("letter is {}",letter),
//         's'=> println!("letter is {}",letter),
//         _ => println!("None"),
//     }
// }                    
// #[derive(Debug)]
// enum Usstate {
//     Alaska,
//     California,
//     Taxes
// }
// #[derive(Debug)]
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Usstate),
// }
// fn value(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {println!("Lucky Penny");
//         1},
//         Coin::Nickel => {println!("Best Nickel");
//         5},
//         Coin::Dime => {println!("Good Dime");
//         10},
//         Coin::Quarter(state) => {println!("Fortunate Quarter of {:?}",state);
//         25},
//     }
// }
// fn main() {
//     let coin1 = Coin::Penny;
//     println!("The Value OF This Coin Is {} cents",value(coin1));
//     let coin2 = Coin::Quarter(Usstate::Taxes);
//     println!("The Value OF This Coin Is {} cents",value(coin2));
//     let coin3 = Coin::Dime;
//     println!("The Value OF This Coin Is {} cents",value(coin3));
//     let coin4 = Coin::Nickel;
//     println!("The Value OF This Coin Is {} cents",value(coin4));
// }

// fn main() {
//     let number = Some(2);
//     println!("{:?}",square(number));
//     let null: Option<i32> = None;
//     println!("{:?}",null);
// }
// fn square(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i*i),
//         None => None
//     }
// }

