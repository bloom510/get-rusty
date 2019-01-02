use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // On the same thread, generate a random number between 1 and 100 
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Uncomment the line below to view the number in your console
    // println!("The secret number is {}", secret_number);

    // Runs the program until the user wins
    loop {
        println!("Please input your guess.");

        // guess is a mutable variable of type String
        let mut guess = String::new();

        /* stdin() returns an instance upon which we may call read_line.
           In ths case, it takes a ref to guess as an argument */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /* Shadow the previous value of guess with a new value. 
           It will parse guess into an unsigned 32-bit integer 
           and gracefully handle errors without crashing the program. */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}", guess);
        
        /* compare the value of guess with that of the secret number and use 
           std::cmp::Ordering to handle the outcomes */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
