use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, welcome to the guessing game");
    println!("I am thinking a number between 1 -100.Can you guess it");
    //Generate number between 1- 100

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Plaese input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faild to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=> {
                println!("Error parsing the guess");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small try again"),
            Ordering::Greater => println!("Too big try again"),
            Ordering::Equal => {
                println!("Congratulations you guessed the number");
                break;
            }
        }
    }
}
