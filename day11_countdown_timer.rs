use std::{io::{self, Write}, thread, time::Duration};

fn main() {
    println!("Basic Timer Tool");
    println!("Enter Timer duration (format: hours minute seconds)");

    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!("Invalid input. Please enter numbers only. (e.g : 0 1 30)");
            return;
        }
    };

    println!("Timer set for {} hours, {} minute, {} seconds", duration.0, duration.1, duration.2);

    start_timer(duration.0, duration.1,duration.2);

    println!("Times Up!!!");

}

fn start_timer(hours: u64,minute: u64, seconds: u64){
    let total_seconds = hours* 3600 + minute * 60 + seconds;

    for i in (1..total_seconds).rev() {
        let hrs = i / 3600;
        let mins = (i % 3600)/60;
        let secs = i % 60;

        print!("\r Time Remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!()
}



fn get_timer_input() -> Option<(u64,u64,u64)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read the line!");

    let parts:Vec<&str> = input.trim().split_whitespace().collect();

    if(parts.len() != 3){
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;    
    let minutes = parts[1].parse::<u64>().ok()?;    
    let seconds = parts[2].parse::<u64>().ok()?;    

    Some((hours,minutes,seconds))

}
