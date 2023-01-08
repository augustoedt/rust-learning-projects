use std::env::{args, Args};

fn main(){
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result = operate(operator, first_number, second_number);
    output(operator, first_number, second_number, result)
}

fn operate(operator: char, first_number:f32, second_number:f32) -> f32 {
    // if operator == '+' {
    //     first_number + second_number
    // } else if operator == '-' {
    //     first_number - second_number
    // } else if operator == '/' {
    //     first_number / second_number
    // } else if operator == '*' {
    //     first_number * second_number
    // } else {
    //      0.0
    // }
    match operator {
        '+' => first_number+second_number,
        '-' => first_number-second_number,
        '/' => first_number/second_number,
        '*' => first_number*second_number,
        _ =>  panic!("invalid operator used")
    }
}

fn output(operator: char, first_number:f32, second_number:f32, result:f32){
    println!("{} {} {} = {}",first_number,operator,second_number, result);
}