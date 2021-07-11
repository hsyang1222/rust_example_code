/*
이종찬 코딩 문제
1이상 100 이하의 정수 n, m을 입력 받아 n, m의 최소공배수와 최대공약수를 출력하는 프로그램을 작성하라.(범위를 벗어나는 숫자를 입력할 경우, 다시 입력 받도록 처리한다.)
*/

use std::io;

fn gcd(n: u32, m: u32) -> u32
{
    if m == 0
    {
        return n;
    }
    else
    {
        return gcd(m, n % m);
    }
}

fn main() {
    loop
    {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: u32 = n.trim().parse().expect("Please type a number");

        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("Failed to read line");
        let m: u32 = m.trim().parse().expect("Please type a number");

        if n < 1 || n > 100 || m < 1 || m > 100
        {
            continue;
        }

        let gcd = gcd(n, m);
        println!("LCD: {}", n * m / gcd);
        println!("GCD: {}", gcd);

        break;
    }
}
