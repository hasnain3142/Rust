 fn main() 
 {
     println!{"Practice Of Chapter 1, 2 and 3 Completed"};
 }                                                     
                                                      
                                                      
                                                       //"Hello, World"//
// fn main() {
//     println!{"Hello, World!"};
// }

                                                        //Hello, Cargo//
// fn main() {
//     println!{"Hello, Cargo"};
// }

                                                        //Variables//
// fn main()
// {
//     let mut x = 110;
//     println!{"The Value Of x Is: {}", x};
//     x = 786;
//     println!{"The Value OF x Is: {}", x};
// }              

                                                        //Constants//
//  const SPEED_OF_SOUND: i32 = 332;
// fn main() 
// {
//        println!{"Your Answer Is {}", SPEED_OF_SOUND};
    
// }              

                                                        //Shadowing//
// fn main() 
// {
//     let x = 12;
//     let x = x + 8;
//     let x = x*5;
//     println!{"The Value Of x Is: {}", x};
// }       

                                                         //Data Types//
// Scalar Data Type
// fn main() {
//     let x: u32 = 110;
//     println!{"The Answer Is: {}", x };
//     let y : f64 = 1.10;
//     println!{"The Answer Is: {}", y };
//     let z : bool = true;
//     println!{"The Answer Is: {}", z };
//     let a : char = 'H';
//     println!{"The Answer Is: {}", a };
// }                                                         

// Number Literals
// fn main() {
//     let Decimal = 134;
//     println!{"The Answer Is: {}", Decimal};
//     let Hex = 0x20A;
//     println!{"The Answer Is: {}", Hex};
//     let Octal = 0o740;
//     println!{"The Answer Is: {}", Octal};
//     let binary = 0b110110;
//     println!{"The Answer Is: {}", binary};
//     let byte = b'H';
//     println!{"The Answer Is: {}", byte};

// }

                                                       //Arithmetic Operations
// fn main() 
// {
//     let x = 12.0;
//     let y = 5.0;

//     let sum = x+y;
//     println!{"{}",sum};
    
//     let minus = y-x;
//     println!{"{}",minus};

//     let multiply = x*y;
//     println!{"{}",multiply};

//     let div = x/y;
//     println!{"{}",div};

//     let rem = x%y;
//     println!{"{}",rem};
    
//     println!{"Mission Completed"};
// }
                                                                   //Compound Data Types
 //Tuple
// fn main() 
// {

//    let tuple : (u32, f64, char, bool) = (17, 83.8, 'A', true);  
//    println!{"The Student Data Is: {:#?}", tuple};
// }
//    println!{"The Age Of Student Is: {}", tuple.0};
//    println!{"The Percentage Of Student Is: {}", tuple.1};
//    println!{"The Grade Of Student Is: {}", tuple.2};
//    println!{"The Student Has Passed: {}", tuple.3};
// Destructure
// let (a, b , c, d) = tuple;

//    println!{"The Age Of Student Is: {}", a};
//    println!{"The Percentage Of Student Is: {}", b};
//    println!{"The Grade Of Student Is: {}", c};
//    println!{"The Student Has Passed: {}", d};

// }

//Array

// fn main () 
// {
//     let array : [u32 ; 10] = [0,1,2,3,4,1_1,6,7,8,9];
   
//     println!{"The Value Of Index Is {}",array[5]};

// }

// fn main () 
// {
//     let array : [u32 ; 10] = [0,1,2,3,4,1_1,6,7,8,9];
   
//     println!{"The Array Is {:#?}",array};

// }
// fn main() 
// {
//     let merit_number = [134;32];
//     println!{"{:#?}",merit_number};

// }
                                                                //Functions
// fn main() 
// {
//     println!{"Friend 1 asking for importants"};
//         importants();

//     println!{"Friend 2 asking for importants"};
//         importants();

//     println!{"Friend 3 asking for importants"};
//         importants();

//     println!{"Friend 4 asking for importants"};
//       importants();

    
// }    

// fn importants() 
// {
//     println!{"Calculus"};
//     println!{"Integration"};
//     println!{"Conics"};
// }

// fn main() 
// {
//     square(12, 16);
// }

// fn square(x:u32 , y:u32) 
// {
//     let result_1 = x*x;
//     let result_2 = y*y;
    
//     println!{"The Square Of {} is: {}", x,result_1};
//     println!{"The Square Of {} is: {}", y,result_2};
// }
                                                                    //Statements and Expressions
// fn main() 
// {
//     let ali = {
//         let a = 100;
//         a + 10
//     };
//         println!{"Answer: {}",ali};
// }
                                                                    //Arguments and Parameters
// fn main() 
// {
//     let (value1, value2) = square(12,16);
//     println!{"{},{}", value1,value2};
// }
// fn square(x:u32,y:u32) -> (u32 , u32)
// {
//     let result1 = x*x;
//     let result2 = y*y;
//     (result1,result2)
// }
                                                                    //Control Flow /If Else
// fn main() 
// {
//     let num = 5;
//     if num >= 5 {
//         println!{"Number Is Greater Than or Equal to 5 and It is {}", num};
//     }
//     else {
//         println!{"Number is Smaller than or 5 and It is {}", num};
//     }
// }
// fn main() 
// {
//     let num = 49;

//     if num % 2 == 0 {
//         println!{"The Number Is Divisible By 2"};
//     }
//     else if num % 3 == 0 {
//         println!{"The Number Is Divisible By 3"};
//     }
//     else if num % 4 == 0 {
//         println!{"The Number Is Divisible By 4"};
//     }
//     else if num % 5 == 0 {
//         println!{"The Number Is Divisible By 5"};
//     }
//     else {
//         println!{"The Number Is Not Divisible By 2,3,4,5"};
//     }
// }
// fn main() 
// {
//     let hasnain = 0;
//     let hasnayn = 5;
//     if hasnain == 1 {
//         println!{"Hasnain Is Best"};
//     }
//     else if hasnayn != 0 {
//         println!{"Hasnayn Is Best"};
//     }
// }
// fn main() {
//     let condition = false;
//     let number = if condition {
//         5
//     }
//     else {
//         6
//     };
//     println!{"The Value Of Number Is: {}",number};
// }

// fn main() {
//     let passed = false;
    
//     let grade = if passed {
//         'A'
//     }
    
//     else {
//         'F'
//     };

//     println!{"Grade Is: {}", grade};
// }
//  fn main() {
//      let num = -10;
//      if num < 0 {
//          println!{"The number is negative"}
//      }
//      else if num == 0 {
//          println!{"The Number Is Zero"}
//      }
//     else {
//         println!{"The number is positive"}
//      }
//  }
// fn main() {
//     let even = false;
//     let number = if even {
//         1
//     }
//     else {
//         0
//     };
//     println!{"The number is: {}", number};
// }
                                                                                    //Loops
// fn main()
// {
//     let mut counter = 0;
//     loop {
//         counter = counter + 1;
//         println!{"Hello, World {}", counter};
//         if counter == 100 {
//             break
//         }
//     }
// }
// fn main () 
// {
//     let mut counter = 0;
//     let value = loop {
//             counter = counter + 1;
//             println!{"Hello, Worlds"};
//             if counter ==5 {
//                 break counter
//             }
//         };
//                  println!{"{}",value};
// }
//While Loop
// fn main()
// {
//     let mut husky = 0;
//     while husky < 3 {
//         println!{"Hello, World"};
//         husky = husky + 1;
//     }
// }
// fn main() 
// {
//     let mut meter = 0;
//     let array = [0,1,2,3,4,5,6,7,8,9];
//     while meter < array.len() {
//         println!{"Hello, World No: {}",array[meter]};
//         meter = meter + 1;
//     }
// }
//for loop
// fn main() 
// {
//     for a in (0..10).rev() {
//         println!{"Hello, World {}",a};
//     }
// }
// fn main() {
// let array = [0,1,2,3,4,5,6,7,8,9];
// for a in array.iter() {
//     println!{"The Number Is: {}",a};
// }
// }
