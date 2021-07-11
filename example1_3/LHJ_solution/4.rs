//이종찬 코딩 문제
// 1이상 100 이하의 정수 n, m을 입력 받아 n, m의 최소공배수와 최대공약수를 출력하는 프로그램을 작성하라.
// (범위를 벗어나는 숫자를 입력할 경우, 다시 입력 받도록 처리한다.)
use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("Pleas Big num and small num");

    io::stdin()
        .read_line(&mut a)
        .expect("Faild");

    io::stdin()
        .read_line(&mut b)
        .expect("Faild");

    let a :i16 = a.trim().parse().expect("num!!!");
    let b :i16 = b.trim().parse().expect("num!!!");

    println!("{}", gcd(a,b));
    println!("{}", lcm(a,b));
}

fn gcd(a: i16, b: i16) -> i16 {
    loop{
        let mut r: i16 = a % b;
        let a=b;
        let b=r;
        if b == 0{ break}
        println!("{} {}", a,b);

    }
    a
}

fn lcm(a: i16, b: i16) -> i16 {
    (a*b)/gcd(a,b)
}
