use std::io;

fn main() {
    println!("Please input fahrenheit");
    let mut fah : String = String::new();
    io::stdin()
        .read_line(&mut fah)
        .expect("input error");

    let fah : f64 = fah.trim().parse().expect("convert error");
    let fah : f64 = fah_to_cel(fah);

    println!("Please input celsiusfi");
    let mut cel : String = String::new();
    io::stdin()
        .read_line(&mut cel)
        .expect("input error");

    let cel : f64 = cel.trim().parse().expect("convert error2");
    let cel : f64 = cel_to_fah(cel);

    println!("converted cel : {fah}, converted fah : {cel}");
}

fn fah_to_cel(target:f64)->f64{
    (target-32.0) * 5.0 / 9.0
}

fn cel_to_fah(target:f64)->f64{
    (target) * 1.8 + 32.0
}