// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
use std::io;
use rand::Rng;

fn main() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..101);

  println!("The secret number is {}", secret_number);

  println!("Please input your guess.");

  let mut guess = String::new();
    
  io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
  println!("You guessed {}",guess);
}
