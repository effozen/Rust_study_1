use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("----------Guesssing Game------------");

	// random number part

	let secret_number = rand::thread_rng().gen_range(1..=100);
	println!("Secret_number : {secret_number}");


	// user input and compare the matchness
	loop {
		let mut guess = String::new();
		println!("Input your guessed number");
		io::stdin().read_line(&mut guess).expect("Failed Reading");
		println!("Guessed number : {guess}");
        let guess : u32 = match guess.trim().parse(){
            Ok(number) => number,
			Err(_) => break,
		}; 
		match guess.cmp(&secret_number){
            Ordering::Greater => println!("Too big!"),
			Ordering::Less => println!("Too small!"),
			Ordering::Equal => { 
				println!("You win!");
		        break;
			}
		}
	}
}
