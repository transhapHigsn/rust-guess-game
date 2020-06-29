use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut score: f64 = 100.0;
    const ALLOWED_ATTEMPTS: u32 = 3;

    let mut attempts: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);

        attempts += 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                if attempts > ALLOWED_ATTEMPTS {
                    score = 0.9 * score;
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                if attempts > ALLOWED_ATTEMPTS {
                    score = 0.9 * score;
                } 
            },
            Ordering::Equal => {
                println!("You win! Your score is: {:.1$}", score, 2);
                break;
            }
        }

    }
}
