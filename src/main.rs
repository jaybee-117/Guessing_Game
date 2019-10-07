use std::io

fn main(){
	println!("Welcome to the guessing game!!\n");
	println!("Enter your guess number:");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
               .expect("Couldn't read line");
    println!("You guessed : {}", guess);
}