use std::io

fn main(){
	println!("Welcome to the guessing game!!\n");
	println!("Enter your guess:");
	io:: .read_line(&mut guess)
         .expect("Couldn't read line");
}