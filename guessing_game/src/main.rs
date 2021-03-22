use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    let mut guess_num = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_num += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_num += 1;
            }
            Ordering::Equal => {
                println!("You win after {} guess(es)!", guess_num);
                break;
            }
        }
    }
}
