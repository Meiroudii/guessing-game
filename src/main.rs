use std::io;

//TODO: Add a difficulty

fn main() {
    println!("GUESS the NumberV2");
    println!("Enter your username: ");

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Error: username is not allowed");

    println!("Hello {username}, let's start the guessing game!");
    println!("Enter your guess (1-100)");
}
