use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut tries = 0;
    loop {
        let mut guess = String::new();
        println!("Please input your guess (1-100).");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tries += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                let score = match tries {
                    1 => 10,
                    2 => 8,
                    3 => 5,
                    _ => 3,
                };
                println!("Your score is: {}", score);
                break;
            }
        }

        if tries == 5 {
            println!("You lose! The secret number was: {}", secret_number);
            println!("Your score is: 0");
            break;
        }
    }
}
