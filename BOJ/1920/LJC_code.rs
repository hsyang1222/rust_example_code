/****************************************************************************************
Question)
	N개의 수가 주어졌을 때, 이를 오름차순으로 정렬하는 프로그램을 작성하시오.
input)
	첫째 줄에 수의 개수 N(1 ≤ N ≤ 1,000)이 주어진다. 둘째 줄부터 N개의 줄에는 수 주어진다.
	이 수는 절댓값이 1,000보다 작거나 같은 정수이다. 수는 중복되지 않는다.
output)
	첫째 줄부터 N개의 줄에 오름차순으로 정렬한 결과를 한 줄에 하나씩 출력한다.
*****************************************************************************************/
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut arr = Vec::new();
    let mut arr_len = 0;

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        if num1 < 1 || num1 > 1000 {
            println!("re-enter number1");
        }else{
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut rand_num = rand::thread_rng().gen_range(1,1001);

        loop{
            if arr.contains(&rand_num) {
                rand_num = rand::thread_rng().gen_range(1,1001);
            }else{
                arr.push(rand_num);
                break;
            }
        }
    }

    arr.sort();

    for i in &arr {
        println!("{}", i);
    }
}


/****************************************************************************************
Question)
	N개의 수가 주어졌을 때, 이를 오름차순으로 정렬하는 프로그램을 작성하시오.
input)
	첫째 줄에 수의 개수 N(1 ≤ N ≤ 1,000,000)이 주어진다. 둘째 줄부터 N개의 줄에는 수가 주어진다.
	이 수는 절댓값이 1,000,000보다 작거나 같은 정수이다. 수는 중복되지 않는다.
output)
	첫째 줄부터 N개의 줄에 오름차순으로 정렬한 결과를 한 줄에 하나씩 출력한다.
*****************************************************************************************/
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut arr = Vec::new();
    let mut arr_len = 0;

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        if num1 < 1 || num1 > 1000000 {
            println!("re-enter number1");
        }else{
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut rand_num = rand::thread_rng().gen_range(1,1000001);

        loop{
            if arr.contains(&rand_num) {
                rand_num = rand::thread_rng().gen_range(1,1000001);
            }else{
                arr.push(rand_num);
                break;
            }
        }
    }

    arr.sort();

    for i in &arr {
        println!("{}", i);
    }
}


/****************************************************************************************
Question)
	N개의 수가 주어졌을 때, 이를 오름차순으로 정렬하는 프로그램을 작성하시오.
input)
	첫째 줄에 수의 개수 N(1 ≤ N ≤ 10,000,000)이 주어진다.
	둘째 줄부터 N개의 줄에는 숫자가 주어진다. 이 수는 10,000보다 작거나 같은 자연수이다.
output)
	첫째 줄부터 N개의 줄에 오름차순으로 정렬한 결과를 한 줄에 하나씩 출력한다.
*****************************************************************************************/
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut arr = Vec::new();
    let mut arr_len = 0;

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        if num1 < 1 || num1 > 10000000 {
            println!("re-enter number1");
        }else{
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let rand_num = rand::thread_rng().gen_range(1,10001);
        arr.push(rand_num);
    }

    arr.sort();

    for i in &arr {
        println!("{}", i);
    }
}

/****************************************************************************************
Question)
	숫자 카드는 정수 하나가 적혀져 있는 카드이다. 상근이는 숫자 카드 N개를 가지고 있다.
	정수 M개가 주어졌을 때, 이 수가 적혀있는 숫자 카드를 상근이가 가지고 있는지 아닌지를 구하는 프로그램을 작성하시오.
input)
	첫째 줄에 상근이가 가지고 있는 숫자 카드의 개수 N(1 ≤ N ≤ 500,000)이 주어진다. 둘째 줄에는 숫자 카드에 적혀있는 정수가 주어진다.
	숫자 카드에 적혀있는 수는 -10,000,000보다 크거나 같고, 10,000,000보다 작거나 같다. 두 숫자 카드에 같은 수가 적혀있는 경우는 없다.
	셋째 줄에는 M(1 ≤ M ≤ 500,000)이 주어진다.
	넷째 줄에는 상근이가 가지고 있는 숫자 카드인지 아닌지를 구해야 할 M개의 정수가 주어지며, 이 수는 공백으로 구분되어져 있다.
	이 수도 -10,000,000보다 크거나 같고, 10,000,000보다 작거나 같다.
output)
	첫째 줄에 입력으로 주어진 M개의 수에 대해서, 각 수가 적힌 숫자 카드를 상근이가 가지고 있으면 1을, 아니면 0을 공백으로 구분해 출력한다.
*****************************************************************************************/
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut arr = Vec::new();
    let mut arr2 = Vec::new();
    let mut arr_len = 0;

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if num1 < 1 || num1 > 500000 {
            println!("re-enter number1");
        } else {
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut rand_num = rand::thread_rng().gen_range(-10000000, 10000001);
        loop {
            if arr.contains(&rand_num) {
                rand_num = rand::thread_rng().gen_range(-10000000, 10000001);
            } else {
                arr.push(rand_num);
                break;
            }
        }
    }

    for i in &arr {
        print!("{} ", i);
    }
    println!();

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if num1 < 1 || num1 > 500000 {
            println!("re-enter number1");
        } else {
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut rand_num = rand::thread_rng().gen_range(-10000000, 10000001);
        loop {
            if arr2.contains(&rand_num) {
                rand_num = rand::thread_rng().gen_range(-10000000, 10000001);
            } else {
                arr2.push(rand_num);
                break;
            }
        }
    }

    for i in &arr2 {
        print!("{} ", i);
    }
    println!();
    
    for i in arr2{
        if arr.contains(&i){
            println!("1");
        }else{
            println!("0");
        }
    }
}

/****************************************************************************************
Question)
	N개의 정수 A[1], A[2], …, A[N]이 주어져 있을 때, 이 안에 X라는 정수가 존재하는지 알아내는 프로그램을 작성하시오.
input)
	첫째 줄에 자연수 N(1 ≤ N ≤ 100,000)이 주어진다. 다음 줄에는 N개의 정수 A[1], A[2], …, A[N]이 주어진다.
	다음 줄에는 M(1 ≤ M ≤ 100,000)이 주어진다. 다음 줄에는 M개의 수들이 주어지는데, 이 수들이 A안에 존재하는지 알아내면 된다.
	모든 정수의 범위는 -231 보다 크거나 같고 231보다 작다.
output)
	M개의 줄에 답을 출력한다. 존재하면 1을, 존재하지 않으면 0을 출력한다.
*****************************************************************************************/
use std::io;

fn main() {
    let mut arr = Vec::new();
    let mut arr2 = Vec::new();
    let mut arr_len = 0;

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        if num1 < 1 || num1 > 100000 {
            println!("re-enter number1");
        } else {
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        arr.push(num1);
    }

    for i in &arr {
        print!("{} ", i);
    }
    println!();

    loop {
        let mut num1 = String::new();
        println!("input number");
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        if num1 < 1 || num1 > 100000 {
            println!("re-enter number1");
        } else {
            arr_len = num1;
            break;
        }
    }

    for i in (0..arr_len) {
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("fail");
        let num1: i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        arr2.push(num1);
    }

    for i in &arr2 {
        print!("{} ", i);
    }
    println!();

    for i in arr2{
        if arr.contains(&i){
            println!("1");
        }else{
            println!("0");
        }
    }
}
