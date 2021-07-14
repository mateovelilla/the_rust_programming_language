// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..101);
  loop {
    println!("Please input your guess.");
  
    let mut guess = String::new();
      
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse(){
      Ok(num) => num,
      Err(_) => continue, 
    };
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too Small"),
      Ordering::Greater => println!("Too great"),
      Ordering::Equal => {
        println!("You Win!");
        break;
      },
    }
  }
}
