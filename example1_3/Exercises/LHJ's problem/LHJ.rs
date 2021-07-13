use std::io;
//use std::cmp::Ordering;

fn main() {
    println!("Please input type u32 and f64.");

    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a: u32 = a.trim().parse()
        .expect("Please type a u32!");

    let a = convert(a);




    let mut b = String::new();

    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    let b: f64 = b.trim().parse()
        .expect("Please type b f64!");




    println!("output: {}", a+b);

}

fn convert(x: u32) -> f64 {
    x as f64
}