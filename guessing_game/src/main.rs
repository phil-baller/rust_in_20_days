use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    println!("Please input your guess.");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
