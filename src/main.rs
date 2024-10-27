use rand::Rng;
use std::cmp::Ordering;
use std::io;

//TODO: Add a difficulty

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("GUESS the NumberV2");
    println!("Enter your username: ");

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Error: username is not allowed");

    println!("Hello {username}, let's start the guessing game!");

    loop {
        let bot_guess = rand::thread_rng().gen_range(1..=100);
        println!("Enter your guess (1-100)");

        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Invalid input!");
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {user_guess} vs the bot's guess is {bot_guess} and the secret number is {secret_number}!!");

        match user_guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!\nBot's guess: {bot_guess}");
                if secret_number == bot_guess {
                    println!("The bot guess first!\nYou lose\n{secret_number}");
                    break;
                }
            }
            Ordering::Greater => {
                println!("Too Big!\nBot's guess: {bot_guess}");
                if secret_number == bot_guess {
                    println!("The bot guess first!\nYou lose\n{secret_number}");
                    break;
                }
            }
            Ordering::Equal => {
                println!("You win!");
                if secret_number == bot_guess {
                    println!("You've unlocked the hidden ending!!: Both bot and the player wins!");
                }
                break;
            }
        }
    }
}
