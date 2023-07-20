use std::io;

fn main() {
    println!("please input the number");

    let mut input_number : String = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("error");

    let input_number : u64 = input_number.trim().parse().expect("parse error");

    let input_number : u64 = generate_fibonacci(input_number);

    println!("result : {input_number}");
}

fn generate_fibonacci(i : u64) -> u64{
    let mut f : u64 = 1;
    let mut b : u64 = 1;
    let mut tmp : u64 = 2;
    if i == 1 {
        b
    } else if i == 2 {
        b
    } else {
        for repeat in (3..=i){
            tmp = b;
            b = f + b;
            f = tmp;
        }
        b
    }
}