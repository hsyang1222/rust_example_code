/*********************************************************************
수학시험에서 진은 30점, 뷔는 80점, 지민은 60점, 희준은 시험을 보지 않았다.
시험을 본 사람들에 한해 각 인원과 점수를 해쉬맵을 이용하여 구현한 후
시험을 안보았을 경우에만 0점이 삽입되도록 프로그램을 구현하여 이름과 점수를 출력.
(시험을 안본사람이 누구인지 몰라 or_insert를 사용하여 안본사람만 0점을 넣어주시오.)
**********************************************************************/
#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Jin"), 30);
    scores.insert(String::from("V"), 80);
    scores.insert(String::from("Jimin"), 60);

    scores.entry(String::from("Jin")).or_insert(0);
    scores.entry(String::from("V")).or_insert(0);
    scores.entry(String::from("Jimin")).or_insert(0);
    scores.entry(String::from("Huijun")).or_insert(0);

    println!("{:?}", scores);
}