use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..7);

    loop{
    let mut guess = String::new();

    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess:u32 = guess.trim().parse().expect("Please type a number");

    //println!("Secret: {}", secret);
    println!("Your guessed: {}", guess);
    match guess.cmp(&secret) {
        Ordering::Less => println!("Less than the secret"),
        Ordering::Greater => println!("Greater than the secret"),
        Ordering::Equal => {
            println!("Equal to the secret");
            break;
            
    
        },
    }
}
}
