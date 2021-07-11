/*
이희준 코딩 문제
x, y (x:정수 y:소수)를 임의로 입력받아 계산(덧셈, 나눗셈 등 자유)하여 출력하라
*/

use std::io;

fn check_prime_number(num: u32) -> bool
{
    for i in 2..num
    {
        if num % i == 0
        {
            return false;
        }
    }
    
    return true;
}

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: u32 = x.trim().parse().expect("Please type a number");

    let mut y = String::new();

    loop
    {
        io::stdin().read_line(&mut y).expect("Failed to read line");
        let y: u32 = y.trim().parse().expect("Please type a number");

        let ret = check_prime_number(y);
        if ret == true
        {
            break;
        }

        println!("Please input a prime number");
    }

    let y: u32 = y.trim().parse().expect("Please type a number");

    println!("{} + {} = {}", x, y, x+y);
    println!("{} - {} = {}", x, y, x-y);
    println!("{} * {} = {}", x, y, x*y);
    println!("{} / {} = {}", x, y, x/y);
}
