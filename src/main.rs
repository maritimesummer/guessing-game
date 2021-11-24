use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main(){
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Please input your guess: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}, the number is {}", "Too small!".red(), secret_number),
            Ordering::Greater => println!("{}, the number is {}", "Too big!".red(), secret_number),
            Ordering::Equal => println!("{}", "You win!".green())
        }
    }
}