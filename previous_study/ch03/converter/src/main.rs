use std::io;

fn main() {
	// Read the user input 
	// User must be input twice, first is for c or f and next is the number
	let mut user_input = String::new();

	println!("Please input the c or f");
	io::stdin()
		.read_line(&mut user_input)
		.expect("Err : Cannot read the line");

	let user_input = user_input.trim().to_lowercase();

	let mut user_number = String::new();

	println!("Please input the number that you wanna convert");
	io::stdin()
		.read_line(&mut user_number)
		.expect("Err : Cannot read the line");

	let user_number : f64 = match user_number.trim().parse(){
		Ok(num) => num,
		Err(_) => 0.0 ,
	};

	let mut result : f64 = 0.0;

	if user_input == "c"{
		result = convert_c_to_f(user_number);
	}else if user_input == "f" {
		result = convert_f_to_c(user_number);
	}else{
		println!("Only c,C,f,F are allowed.");
	}
	println!("Result : {result}");
}

fn convert_c_to_f(c:f64)->f64{
	c*9.0/5.0+32.0
}

fn convert_f_to_c(f:f64)->f64{
	(f-32.0)*5.0/9.0
}
