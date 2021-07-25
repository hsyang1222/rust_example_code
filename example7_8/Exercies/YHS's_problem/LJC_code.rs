/*****************************************************************************
사용자로부터 무한히 낱말을 입력받는다. 사용자가 end라고 타이핑하여 입력하면 입력은 종료된다.
프로그램은 입력된 각 낱말과, 해당 낱말이 몇번 입력되었는지를 연이어 출력해준다. 이때 사용자가 마지막으로 입력한 end는 낱말로 치지 않는다.
출력할때 apple과 banana 중 무엇이 먼저 나오는지는 중요하지 않고, 단지 입력된 횟수만 정확하면 된다.
******************************************************************************/
use std::io;
use std::collections::HashMap;

fn main() {
    let mut word_set = HashMap::new();

    println!("enter word");

    loop {
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("fail");
        let str = str.trim().to_string();
        if str.eq("end") {
            break;
        }

        let count = word_set.entry(String::from(str)).or_insert(0);
        *count += 1;
    }

    for (key,value) in word_set {
        println!("{} : {}", key, value);
    }
}
