/*********************************************************************
수학시험에서 진은 30점, 뷔는 80점, 지민은 60점, 희준은 시험을 보지 않았다.
시험을 본 사람들에 한해 각 인원과 점수를 해쉬맵을 이용하여 구현한 후
시험을 안보았을 경우에만 0점이 삽입되도록 프로그램을 구현하여 이름과 점수를 출력.
(시험을 안본사람이 누구인지 몰라 or_insert를 사용하여 안본사람만 0점을 넣어주시오.)
**********************************************************************/
use std::io;
use std::collections::HashMap;

fn main() {
    let mut exam = HashMap::new();

    exam.insert(String::from("진"), 30);
    exam.insert(String::from("뷔"), 80);
    exam.insert(String::from("지민"), 60);

    exam.entry(String::from("진")).or_insert(0);
    exam.entry(String::from("뷔")).or_insert(0);
    exam.entry(String::from("지민")).or_insert(0);
    exam.entry(String::from("희준")).or_insert(0);

    for (key, value) in &exam {
        println!("{}'s grade {}", key, value);
    }
}
