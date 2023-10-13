use std::io;

fn main() {

    let mut first_number = String::new();
    println!("Input the first number");
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Invalid number");

    let mut input = String::new();
    println!("Input the operator (+, -, *, /)");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let mut second_number = String::new();
    println!("Input the second number");
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Invalid number");

    let result: f64 = match input {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => panic!("Invalid operator"),
    };

    println!("Result: {}", result);
    // let x = 4;
    // println!("x is {}", x);

    // let floating_point = 10.92;
    
    // println!("floating_point is {}", floating_point);
    // let my_tuple: (i32, f64, &str) = (42, 3.14, "hello");

    // // Format the values as a single string and print it
    // let formatted_string = format!("Tuple values: {}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2);
    // println!("{}", formatted_string);
    
    // //array

    // let arr: [i32; 5] = [1,6,9,4,5];

    // println!("{}", arr[2]);
   

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     println!("counter: {counter}");
    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");
    
    // let a = [10, 20, 30, 40, 50];

    // let mut index = 0;

    // while index < 5 {
    //     println!("The value is : {}", a[index]);
    //     index += 1;
    // }

    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

   
}
