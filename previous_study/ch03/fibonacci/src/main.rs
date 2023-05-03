use std::io;
fn main() {
    println!("Fibbonacci number generater");
	println!("Please input the number that you want to cal");

	let mut input : String = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Err: Cannot read the line.");
	let input : u32 = match input.trim().parse(){
        Ok(num) => num,
		Err(_) => 0,
	};
    let fibo = fibonacci(input);
	println!("Result : {fibo}");
}

fn fibonacci(num:u32)->u32{
	let mut num1 : u32 = 1;
    let mut num2 : u32 = 1;
	let mut tmp : u32 = 0;
    
	if num == 0 {
	    return 0;
	}else if num == 1 {
        return 1;
	}else if num == 2 {
	    return 2;
	}
	
	for _cnt in 3..=num {
        tmp = num2;
		num2 += num1;
		num1 = tmp;
	}
	num2
}
