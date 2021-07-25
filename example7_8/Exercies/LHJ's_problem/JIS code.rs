/*
수학시험에서 진은 30점, 뷔는 80점, 지민은 60점, 희준은 시험을 보지 않았다. 시험을 본 사람들에 한해 각 인원과 점수를 해쉬맵을 이용하여 구현한 후
시험을 안보았을 경우에만 0점이 삽입되도록 프로그램을 구현하여 이름과 점수를 출력. (시험을 안본사람이 누구인지 몰라 or_insert를 사용하여 안본사람만 0점을 넣어주시오.)
*/

use std::collections::HashMap;

fn main() {
    let mut score_map = HashMap::new();

    let score_jin = score_map.entry("Jin").or_insert(0);
    *score_jin = 30;

    let score_v = score_map.entry("V").or_insert(0);
    *score_v = 80;

    let score_jimin = score_map.entry("Jimin").or_insert(0);
    *score_jimin = 60;

    let score_huijun = score_map.entry("Huijun").or_insert(0);

    for (key, value) in score_map
    {
        println!("{}: {}", key, value);
    }
}
