use std::{io};

fn main() {
    println!("Fibbonacci generator");
    println!("Enter fib sequence length");

    let lenght = match get_sequence_length_as_u32() {
        Some(val) => val,
        None => 0
    };


    let sequence = generate_seqence(lenght);

    println!("Fibonacci sequence of lenght {} : {:?}", lenght,sequence);
}

fn generate_seqence(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n>=1 {
        sequence.push(0);
    }
    if n>= 2 {
        sequence.push(1);
    }

    for i in 2..n {
        let next = sequence[i as usize -1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence

}

fn get_sequence_length_as_u32() -> Option<u32> {
    let mut lenght = String::new();

    io::stdin().read_line(&mut lenght).expect("Failed to read line");

    match lenght.trim().parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => None
    }
}


