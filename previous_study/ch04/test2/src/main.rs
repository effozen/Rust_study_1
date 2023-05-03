fn main() {
    let mut s = String::from("Hello!");

	{
        let _r1 = &mut s;
	}

	let _r2 = &mut s;
}
