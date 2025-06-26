use core::num;
use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    
    println!("ENTER YOUR CHOICE (1-2):");

    let mut choice = String::new();

    io::stdin().read_line( &mut choice).expect("Failed to read line");

    let choice :u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };
    if(choice== 1){
        convet_to_fahrenheit();
    }else if (choice == 2){
        convert_to_celsius();
    }else {
        println!("Invalid choice. Please enter 1 or 2.");
        return;
    }
        


}

fn convet_to_fahrenheit(){
    println!("ENTER TEMPERATURE IN CELSIUS:");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");

    let celcuis :f64 = match celsius.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };
    let fahrenheit = (celcuis * 9.0 / 5.0) + 32.0;
    println!("Temperature of {} Celsius is equal to {} Fahrenheit", celcuis, fahrenheit);

}

fn convert_to_celsius(){
    println!("EnTER TEMPERATURE IN FAHRENHEIT:");

    let mut fahrenheight = String::new();
    io::stdin().read_line(&mut fahrenheight).expect("Failed to read line") ;

    let fahrenheight : f64 = match fahrenheight.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ivalid input please enter a number");
            return;
        }
    };

    let celsius = (fahrenheight - 32.0) * 5.0 / 9.0;
    println!("Temperature of {} Fahrenheit is equal to {} Celsius", fahrenheight, celsius);
}


