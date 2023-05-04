// Lim, jaedo a.k.a. OAO | ashgrayblue0@gamil.com | 2023.05.04.Thursday | MIT License

// only use the i32 data type

use std::io;

fn main() {
	// UI

	let mut num1=String::new();
	let mut num2=String::new();
	let mut op=String::new();

	println!("num 1");
	io::stdin().read_line(&mut num1).expect("num1 input error");
	println!("num 2");
	io::stdin().read_line(&mut num2).expect("num2 input error");
	println!("operation");
	io::stdin().read_line(&mut op).expect("num3 input error");


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
