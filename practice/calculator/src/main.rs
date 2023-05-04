// Lim, jaedo a.k.a. OAO | ashgrayblue0@gamil.com | 2023.05.04.Thursday | MIT License

// only use the i32 data type

use std::io;

fn main() {
	calculator();
}

fn user_input()->(i32,i32,char){
	let mut num1=String::new();
	let mut num2=String::new();
	let mut op=String::new();

	println!("num 1");
	io::stdin().read_line(&mut num1).expect("num1 input error");
	println!("num 2");
	io::stdin().read_line(&mut num2).expect("num2 input error");
	println!("operation");
	io::stdin().read_line(&mut op).expect("operation input error");

	let num1 : i32 = num1.trim().parse().expect("num1 parse error");
	let num2 : i32 = num2.trim().parse().expect("num2 parse error");
	let op : char = op.trim().parse().expect("op parse error");

	(num1, num2, op)
}

fn calculator(){
	let mut num1 : i32;
	let mut num2 : i32;
	let mut op : char;
	loop{
		(num1, num2, op) = user_input();
		match(op){
			'+' => {
				println!("result: {}", add(num1,num2));
			},
			'-' => {
				println!("result: {}", sub(num1,num2));
			},
			'*' => {
				println!("result: {}", mul(num1,num2));
			},
			'/' => {
				println!("result: {}", div(num1,num2));
			},
			'%' => {
				println!("result: {}", modu(num1,num2));
			},
			_ => {
				println!("Error");
				break;
			}
		}

	}
}

fn add(x:i32, y:i32)->i64 {
	(x as i64)+(y as i64)
}

fn sub(x:i32, y:i32)->i64{
	(x as i64)-(y as i64)
}

fn mul(x:i32, y:i32)->i64{
	(x as i64)*(y as i64)
}

fn div(x:i32, y:i32)->f64{
	(x as f64)/(y as f64)
}

fn modu(x:i32, y:i32)->i64{
	(x as i64)%(y as i64)
}
