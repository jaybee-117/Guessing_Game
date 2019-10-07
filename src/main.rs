use std::io;
use rand::Rng

fn main(){
	println!("Welcome to the guessing game!!\n");
	let secret_number = thread_rng().gen_range(1,101);
	println!("The secret number is : {}", secret_number);
	println!("Enter your guess number:");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
               .expect("Couldn't read line");
    println!("You guessed : {}", guess);
}