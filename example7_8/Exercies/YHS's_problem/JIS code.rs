/*
word count 프로그램을 만들어보려고 한다. 이 프로그램은 매우 유명한 프로그램이라 검색하면 코드를 쉽게 찾을 수 있다. 하지만, 이번 문제에서는 검색을 이용하지 말고 직접 풀어보아라.

사용자로부터 무한히 낱말을 입력받는다. 사용자가 end라고 타이핑하여 입력하면 입력은 종료된다.
프로그램은 입력된 각 낱말과, 해당 낱말이 몇번 입력되었는지를 연이어 출력해준다. 이때 사용자가 마지막으로 입력한 end는 낱말로 치지 않는다.
예를들어 사용자가 아래와 같이 입력했다고 하자
*/

use std::collections::HashMap;
use std::io;

fn main() {
    let mut word_map = HashMap::new();

    loop
    {
        let mut word = String::new();
        io::stdin().read_line(&mut  word).expect("Failed to read line");

        if word.trim().eq("end") == true
        {
            break;
        }

        let word_num = word_map.entry(word).or_insert(0);
        *word_num += 1;
    }

    for (key, value) in &word_map
    {
        println!("{} : {}", key, value);
    }
}
