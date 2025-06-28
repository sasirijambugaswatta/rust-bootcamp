use std::io::{Read};
use std::env;
use std::fs::File;
fn main() {
    let args: Vec<String> =env::args().collect();

    if args.len() != 2 {
        println!("Invalid usage. cargo run <filepath>");
        return;
    }

    let file_path = &args[1];
    println!("reading file path : {}",file_path);

    //Read content
    let mut file =match File::open(file_path) {
        Ok(file)=> file,
        Err(err) => {
            println!("Error opening file {}",err);
            return;
        }
    };

    let mut content = String::new();
    
    if let Err(err) = file.read_to_string(&mut content){
        println!("Error reading file {}", err);
        return;
    } 

    //count word 
    let word_count = count_words(&content);
    println!("Word coint : {}",word_count);


}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}
