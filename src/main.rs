use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..7);

    loop{
    let mut guess = String::new();

    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Your guessed: {}", guess);
    match guess.cmp(&secret) {
        Ordering::Less => println!("{}", "Less than the secret".red()),
        Ordering::Greater => println!("{}","Greater than the secret".red()),
        Ordering::Equal => {
            println!("{}", "Equal to the secret".green());
            break;
            
    
        },
    }
}
}
