//Ownership
// fn main()
// {
//     let mut h = String::from("Hello,");
//     h.push_str(" World!");
//     println!{"{}",h};
// }
// fn main() 
// {
//     //For Primitive Data Type It Just Copy
//     let x = 5;
//     let y = x;
//     println!{"Value Of x: {} and Value Of y: {}", x, y};
// //But For String It Moves
//     let s1 = String::from("Hasnain");
//     let s2 = s1;
//     // println!{"Value Of s1: {}",s1}; // It is wrong because value has moved
//     println!{"Value Of s2: {}",s2};
// }
//Cloning
// fn main()
// {
//     let s1 = String::from("Software");
//     let s2 = s1.clone();                    //Deep Copy
//         println!{"Value Of s1: {}",s1};
//         println!{"Value Of s2: {}",s2};
// }
// fn main()
// {
//     const STAY: &str = "Hasnain's Ambition";
//     println!{"What Cannot Be Changed: {}",STAY};
// }
//Ownership And Functions

// fn main()
// {
//     let h = String::from("Hasnain");
//     main2(h);
// }
// fn main2(x: String)
// {
//     println!{"Hello, {}",x};
// }
// fn main()
// {
//     let s = 110;
//     main2(s);
//     println!{"Mine {}",s};
// }
// fn main2(x: i32)
// {
//     println!{"Yours {}",x};
// }
// fn main()
// {
//     let h = main2();
//     println!{"{} Hello!", h};
// }
// fn main2() -> String
// {
//     let s = String::from("Hello, World");
//     println!{"{} Hi!", s};
//     s
    
// }
// fn main()
// {
//     let s1 = String::from("Hello, Earth");
//     let s2 = main2(s1);
//     println!{"{}",s2};
// }
// fn main2(x: String) -> String 
// {
//     x
// }
// fn main()
// {
//     let s1 = train();
//     println!{"{}",s1};
// }
// fn train() -> String 
// {
//     let x = String::from("Hello");
//     x
// }
// fn main() 
// {
//     let s2 = String::from("Hello");
//     let s3 = main2(s2);
//     println!{"s3 ka {}",s3};
// }
// fn main2(s1: String) -> String
// {
//     println!{"{}",s1};
//     s1
// }
// fn main()
// {
//     let s1 = String::from("Hello");
//     let (s2, len) = calculate_length(s1);
//     println!{"The Length Of {} is {}", s2, len};
// }

// fn calculate_length(s: String) -> (String, usize)
// {
//     let length = s.len();
//     (s, length)
// }
// fn main()
// {
//     let s1 = String::from("Pakistan");
//     let (s2, s3) = length(s1);
//     println!{"The length of {} is {}",s3,s2};
// }
// fn length(name: String) -> (usize, String)
// {
//     (name.len(), name)
// }
//Borrowing and References
fn main()
{
    let a = "14";
    let b = a;
    let c = b;
        println!{"a: {}, b: {}, c: {}", a,b,c};
}
// fn main()
// {
//     let a = 14;
//     let b = &a;
//     let c = &b;
//     println!{"The address of a is {:p}",b};
//     println!{"The address of b is {:p}",c};
// }
// fn main() 
// {
//     let s = String::from("Hasnain");
//     let a = length(&s);
//     println!{"The lengh of the word {} is {}",s,a};
// }
// fn length(x: &String) -> usize
// {
//     x.len()
// }

// fn main()
// {
//     let mut s = String::from("Hasnain");
//     change(&mut s);
// }
// fn change(x: &mut String)
// {
//     x.push_str(" Ali");
//     println!{"{}",x};
// }
// fn main()
// {
//    let mut s = String::from("Hello");
//     let a = &s;
//     let b = &s;
//     let c = &s;
//     println!{"{}, {}, {}",a,b,c};

//     let d = &mut s;
//     d.push_str(",World");
//     println!{"{}",d};
// }
//dangling refernece error
// fn main()
// {
//   let a = dangle();
//   println!{"{} aja",a};
// }
// fn dangle() -> &String
// {
//     let s = String::from("Hello");
//     &s
// }
//SOloved
// fn main()
// {
//   let a = dangle();
//   println!{"{:p}",&a};
// }
// fn dangle() -> String
// {
//     let s = String::from("Hello");
//     println!{"S ka {:p}",&s};
//     s
// }
