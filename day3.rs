use core::num;
use std::io::{self, stdin};
fn main() {
    println!("Simple calculator");
    println!("Availble operators: + - * /");
    println!("Enter expression in following format: 5 + 3");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Cannot read line");

    let tokens : Vec<&str> = input.trim().split_whitespace().collect();

    if(tokens.len() != 3){
        println!("Invalid input : Follow the format 5 + 3 ")
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing num1");
            return;
        }
    };

    let operator = tokens[1];

    let num2 : f64 = match tokens[2].parse() {
        Ok(num)=> num1,
        Err(_)=> {
            println!("Error parsing num2");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1,num2),
        "-" => substract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Error operator");
            return;
        }
    };
}

fn divide(num1: f64, num2: f64) ->f64 {
    if(num2 == 0.0){
        println!("Cannot divide by 0");
        std::process::exit(1);
    }
    return num1/ num2;
}

fn multiply(num1: f64, num2: f64) -> f64{
    num1* num2
}

fn substract(num1: f64, num2: f64) -> f64 {
    return num1 - num2;
}

fn add(num1: f64, num2: f64) -> f64 {
    return num1+ num2;
}


