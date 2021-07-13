/*
정인수 코딩 문제
서울대학교에 다니고 있는 현식이는 돈을 많이 주는 과외 학생만 받아서 과외를 하고 싶다. 10명의 과외비를 입력하고 과외비를 5만원 이상 주는 학생이 몇 명인지 답을 출력하는 프로그램를 작성하라.
입력 값 타입: 정수형 / 출력 값 타입: 정수형
참조 : https://doc.rust-lang.org/std/string/struct.String.html#method.parse
*/

use std::io;

fn main() {
    let mut count = 0;
    for i in 1..11
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number");

        if input >= 50000
        {
            count += 1;
        }
    }

    println!("{} people", count);
}
