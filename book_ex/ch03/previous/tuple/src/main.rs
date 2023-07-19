fn main() {
	let tup1: (i32, f64, u8) = (500, 6.4, 1);

	let tup = (500, 6.4, 1);

	let (x, y, z) = tup;

	let t1 = tup.0;
	let t2 = tup.1;
	let t3 = tup.2;

	println!("The value of y is: {y}");
}
