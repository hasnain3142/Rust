// #[derive(Debug)]
// struct Book
// {
//     name: String,
//     author: String,
//     price: u16,
//     availability: bool
// }
// fn main()
// {
//     let mut book_1 = Book
//     {
//         name: String::from("The Pisoner Of Zenda"),
//         author: String::from("Alexander"),
//         price: 500,
//         availability: true,
//     };
//     book_1.name = String::from("Alexander Pope");
 
//     println!{"{:#?}",book_1};
    
// }
// #[derive(Debug)]
// struct Book
// {
//     name: String,
//     author: String,
//     price: u16,
//     availability: bool,
// }
// fn main()
// {
//     let book_1 = stance(String::from("Brief History of Time"), String::from("Stephen Hawkings"));
//     println!{"{:#?}",book_1};
// }
// fn stance(name:String, author:String) -> Book
// {Book{
//     name:name,
//     author:author,
//     price: 5000,
//     availability: true,
// }
// }
//Update Syntax
// #[derive(Debug)]
// struct Book
// {
//     name: String,
//     author: String,
//     price: u16,
//     availability: bool
// }
// fn main()
// {
//     let book_1 = Book
//     {
//         name : String::from("Kitaab"),
//         author: String::from("MUsannif"),
//         price: 1000,
//         availability: true,

//     };
//     println!("{:#?}",book_1);

//     let book_2 = Book
//     {
//         name : String::from("Dusri Kitaab"),
//         ..book_1
//     };
//     println!("{:#?}",book_2);
// }
// Tuple Struct
// #[derive(Debug)]
// struct Letters (char,char,char);
// #[derive(Debug)]
// struct Numbers (i32,i32,i32);
// #[derive(Debug)]
// struct Hex (i32,i32,char,char);

// fn main()
// {
//     let stance1 = Letters('a','b','c');
//     println!("{:#?}",stance1);

//     let stance2 = Numbers(0,1,2);
//     println!("{:#?}",stance2);

//     let stance3 = Hex(0,9,'A','F');
//     println!("{:#?}",stance3);
// }
// #[derive(Debug)]
// struct Numbers (i32,i32,i32,i32,i32);

// // fn main()
// // {
// //     let digits = Numbers(0,1,2,3,4);  //Variable Declaration
// //             main2(digits) // Argument
// // }
// // fn main2(x: Numbers) // parameter
// // {
// //     println!("{:#?}",x)
// // }
// fn main()
// {
//     let height = 5;
//     let width = 10;
//     let y = area(height, width);
//     println!("The area of rectangle is: {}",y);
// }
// fn area(height1:i32, width1:i32) -> i32 
// {
//     height1*width1
// }
//Refractoring Tuples
// fn main()
// {
//     let rect1 = (5, 10);
//     let result = area(rect1);
//     println!("The Area OF Rectangle Is: {}",result);
// }
// fn area(dimensions: (u32,u32)) -> u32
// {
//     dimensions.0*dimensions.1
// }

// #[derive(Debug)]
// struct Rectangle
// {
//     height: u32,
//     width: u32,
// }
// fn main()
// {
//     let rec_1 = Rectangle{
//         height: 5,
//         width: 10,
//     };
//     println!("The area of rectangle is {}",area(rec_1));
// }
// fn area(rec1: Rectangle) -> u32
// {
//     rec1.height*rec1.width
// }
                                                                                            //Methods

// #[derive(Debug)]
// struct Rectangle
// {
//     height: u32,
//     width: u32,
// }

// impl Rectangle
// {
//     fn area(&self) -> u32 
//     {
//         self.height*self.width
//     }
// }
// fn main()
// {
//     let rectangle1 = Rectangle {height:25, width:4};
//     println!("The area of rectangle1 is {}",rectangle1.area());
//     let rectangle2 = Rectangle {width:100, height:4};
//     println!("The area of rectangle2 is {}",rectangle2.area());
// }
//Methods With More Than One Parameter

// #[derive(Debug)]
// struct Rectangle
// {
//     height: u32,
//     width: u32,
// }

// impl Rectangle
// {
//     fn meri(&self, other: &Rectangle) -> bool
//     {
//         self.height > other.height && self.width > other.width
//     }
// }
// fn main()
// {
//     let rec1 = &Rectangle {height:2, width:30};
//     let rec2 = &Rectangle {height:10, width:25};
//     let rec3 = &Rectangle {height:50, width:20};

//     println!("Can Rec1 hold Rec2?: {}",rec1.meri(rec2));
//     println!("Can Rec2 hold Rec3?: {}",rec2.meri(rec3));

// }
