use std::io;
use rand::Rng;

fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn is_multiple_or_divisor(n: u32, m: u32) -> bool {
    if m % n == 0 || n % m == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut total_score = 0;
    let mut round = 0;

    while round < 3 {
        round += 1;
        println!("Round {} - Guess the number!", round);

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
                    total_score += score;
                    break;
                }
            }

            if tries == 3 {
                println!("\n\n Look like you need some hint!!\n\n");
                println!("Enter number then, We check the number is multiply or divisor guess number!! (default = 1)\n");
                let mut hint: String = String::new();
                io::stdin().read_line(&mut hint).expect("Failed to read line");
                let hint: u32 = match hint.trim().parse(){
                    Ok(num) => num,
                    Err(_) => 1,
                };
                if is_multiple_or_divisor(secret_number, hint) {
                    println!("\n{} is multiply or divisor of guess number!", hint);
                } else {
                    println!("\n{} is not multiply or divisor of guess number!", hint);
                }
                if is_prime(secret_number){
                    println!("And guess number is prime!\n");
                }else {
                    println!("And guess number is not prime\n");
                }
            }else if tries == 5 {
                println!("You lose! The secret number was: {}", secret_number);
                break;   
            }
        }
    }
    println!("Your total score is: {}", total_score);
}
