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
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }

        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5
}