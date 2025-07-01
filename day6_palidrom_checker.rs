use std::io;

fn main() {
    println!("Palidrome checker");
    println!("Enter word to check: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading line");

    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a valid non empty string");
        return;
    }

    if is_palindrom(&cleaned_input) {
        println!("{} is palindrom! ", input.trim());
    }else {
        println!("{} is not palindrom", input.trim());
    }

}

fn is_palindrom(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn clean_string(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect()
}
