/*
N개의 정수 A[1], A[2], …, A[N]이 주어져 있을 때, 이 안에 X라는 정수가 존재하는지 알아내는 프로그램을 작성하시오.
첫째 줄에 자연수 N(1 ≤ N ≤ 100,000)이 주어진다. 다음 줄에는 N개의 정수 A[1], A[2], …, A[N]이 주어진다. 다음 줄에는 M(1 ≤ M ≤ 100,000)이 주어진다. 다음 줄에는 M개의 수들이 주어지는데, 이 수들이 A안에 존재하는지 알아내면 된다. 모든 정수의 범위는 -231 보다 크거나 같고 231보다 작다.

출력
M개의 줄에 답을 출력한다. 존재하면 1을, 존재하지 않으면 0을 출력한다.

예제 입력 1
5
4 1 5 2 3
5
1 3 7 9 5
예제 출력 1
1
1
0
0
1
*/

use std::io;
use std::collections::{HashMap, HashSet};

fn str_readline() -> String{
    let mut insert_line = String::new();
    io::stdin().read_line(&mut insert_line).expect("read fail");
    insert_line
}

fn str_line_to_int() -> i64{
    let line = str_readline();
    let num = line.trim().parse().expect("parse fail");
    num
}

fn str_line_to_int_vec() -> Vec<i64>{
    let mut vec:Vec<i64> = Vec::new();
    let mut insert_line = str_readline();
    let split_line = insert_line.split_whitespace().map(|x| x.parse().expect("parse error")).collect();
    split_line
}

fn str_line_to_int_hashset() -> HashSet<i64>{
    let mut vec:Vec<i64> = Vec::new();
    let mut insert_line = str_readline();
    let split_line = insert_line.split_whitespace().map(|x| x.parse().expect("parse error")).collect();
    split_line
}

fn main() {
    let n = str_line_to_int();
    let vec_n_num= str_line_to_int_hashset();
    let m = str_line_to_int();
    let vec_m_num:Vec<i64> = str_line_to_int_vec();

    for m_num in vec_m_num{
        if vec_n_num.contains(&m_num){
            println!("1");
        }else{
            println!("0");
        }
    }
}
