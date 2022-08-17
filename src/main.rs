use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number :)");

    //generate a random number in range 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //loop until user guess the correct numner
    loop {
        println!("Please enter your guess");

        let mut guess = String::new(); // by default variables are immutable in RUST, initializing MUTABLE variable called guess

        //read user input
        io::stdin()
            .read_line(&mut guess) //read_line return Ok and Err
            .expect("Failed to read line"); //handle if Result is an Err rather than Ok.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Hooray!, You guessed the number!");
                break;
            }
        }
    }
}
