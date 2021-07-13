/*
양현식 코딩 문제
사용자로부터 숫자 n을 입력 받는다. 1~n까지 숫자 중 소수를 모두 찾아 출력하는 프로그램을 작성하시오.
참조 :  https://doc.rust-lang.org/rust-by-example/flow_control/for.html
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
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number");

    for i in 2..n+1
    {
        let ret = check_prime_number(i);
        if ret == true
        {
            println!("{}", i);
        }
    }
}
