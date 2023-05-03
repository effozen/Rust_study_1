use rand::Rng;
use std::io; 
use std::cmp::Ordering;

fn main() {
	// UI & UX
	println!("My guessing game!");

	// generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

	println!("secret number: {secret_number}");
    println!();

	// loop for read the input and compare the number
	let mut guess = String::new();
	'test : loop {
        // read the user input
		println!("Please input the number: ");
		io::stdin()
			.read_line(&mut guess)
			.expect("Faile to read line;");

        // parse the user input (String) to u32
		let guess = match guess.trim().parse() {
            Ok(num) => num,
			Err(_) => {println!("Parse Err");continue 'test;},
		};

        // Compare the user input
		match secret_number.cmp(&guess){
			Ordering::Less => println!("Too Small!"),
			Ordering::Greater => println!("Too Big!"),
			Ordering::Equal => {println!("You win!"); break 'test;}
		}

	}
}
