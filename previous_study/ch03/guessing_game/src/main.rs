use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
	println!("--------- GUESSING GAME --------------");

	// generate random number
	let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("secret number: {secret_number}");

	'test: loop{
		// read the user's input
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Cannot read the user's input");

		let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
			Err(_) => continue,
	    };

		println!("You guessed: {guess}");

		// Compare the number
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too Small!"),
			Ordering::Greater => println!("Too Big!"),
			Ordering::Equal => {
				println!("You win!");
			    break 'test;
			},
		}
	}
}
