use std::ptr::null;

fn main() {
    // let mut x = 5;
    // println!("x is {}", x);
    //
    // x = 6;
    // println!("x is {}", x);
    // {
    //     fn main() {
    //         let x = 5;
    //
    //         let x = x + 1;
    //
    //         {
    //             let x = x * 2;
    //             println!("The value of x in the inner scope is: {x}");
    //         }
    //
    //         println!("The value of x is: {x}");
    //     }
    //
    //     main();
    // }
    // {
    //     fn main() {
    //         let x = 2.0; // f64
    //
    //         let y: f32 = 3.0; // f32
    //     }
    //     main();
    // }
    // {
    //     fn main() {
    //         // addition
    //         let sum = 5 + 10;
    //
    //         // subtraction
    //         let difference = 95.5 - 4.3;
    //
    //         // multiplication
    //         let product = 4 * 30;
    //
    //         // division
    //         let quotient = 56.7 / 32.2;
    //         let truncated = -5 / 3; // Results in -1
    //         let truncated2 = -5.0 / 3.0; // Results in -1
    //
    //         // remainder
    //         let remainder = 43 % 5;
    //     }
    //     main()
    // }
    // {
    //     fn main() {
    //         let tup: (i32, f64, u8) = (500, 6.4, 1);
    //         println!("{}", tup.0);
    //         println!("{}", tup.1);
    //         println!("{}", tup.2);
    //
    //         let (x, y, z) = tup;
    //
    //         println!("The value of y is: {y}");
    //     }
    //     main()
    // }

    // another_function();

    // println!("{}", five());

    // {
    //     let number = 3;
    //
    //     if number < 5 {
    //         println!("condition was true");
    //     } else {
    //         println!("condition was false");
    //     }
    // }

    // {
    //     let mut counter = 0;
    //
    //     let result = loop {
    //         counter += 1;
    //
    //         if counter == 10 {
    //             break counter * 2;
    //         }
    //     };
    //
    //     println!("The result is {result}");
    // }

    // {
    //     let mut count = 0;
    //     'counting_up: loop {
    //         println!("count = {count}");
    //         let mut remaining = 10;
    //
    //         loop {
    //             println!("remaining = {remaining}");
    //             if remaining == 9 {
    //                 break;
    //             }
    //             if count == 2 {
    //                 break 'counting_up;
    //             }
    //             remaining -= 1;
    //         }
    //
    //         count += 1;
    //     }
    //     println!("End count = {count}");
    // }

    // {
    //     let mut number = 3;
    //
    //     while number != 0 {
    //         println!("{number}!");
    //
    //         number -= 1;
    //     }
    //
    //     println!("LIFTOFF!!!");
    // }
    // {
    //     let a = [10, 20, 30, 40, 50];
    //     let mut index = 0;
    //
    //     while index < 5 {
    //         println!("the value is: {}", a[index]);
    //
    //         index += 1;
    //     }
    //
    //     let a = [10, 20, 30, 40, 50];
    //
    //     for element in a {
    //         println!("the value is: {element}");
    //     }
    //
    //     for number in (1..4).rev() {
    //         println!("{number}!");
    //     }
    //     println!("LIFTOFF!!!");
    // }

    // {
    //     let mut user1 = User {
    //         name: String::from("username"),
    //         age: 13,
    //         email: String::from("example@example.com"),
    //         active: true,
    //     };
    //     let user2 = User {
    //         active: user1.active,
    //         name: user1.name,
    //         email: String::from("another@example.com"),
    //         age: user1.age,
    //     };
    //
    //     let user3 = User {
    //         email: String::from("anotherone@example.com"),
    //         ..user2
    //     };
    // }

    // {
    //     let black = Color(0, 0, 0);
    //     let origin = Point(0, 0, 0);
    // }

    {
        let subject = AlwaysEqual;
    }
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}

struct User {
    name: String,
    age: u8,
    active: bool,
    email: String,
}

fn build_user(name: String, age: u8, active: bool) -> User {
    User {
        name,
        age,
        active,
        email,
    }
}

// 元组结构
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 单元结构
struct AlwaysEqual;
