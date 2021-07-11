use std::io; 

fn main(){ 

    loop{
        let mut n_string = String::new();
        let mut m_string = String::new();

        io::stdin().read_line(&mut n_string).expect("fail read line");
        io::stdin().read_line(&mut m_string).expect("fail read line");

        let n = n_string.trim().parse().expect("fail parse");
        let m = m_string.trim().parse().expect("fail parse");

        if 1<=n && n<=100 && 1<=m && m<=100{
            let mut div_concur = 1;
            for i in 1..101 {
                if n%i==0 && m%i== 0{
                    div_concur = i;
                }
            }

            let mut go_up:i32 = if n>=m { m }else{ n };
            loop{
                if (go_up % n ==0) && (go_up%m == 0){
                    break;
                }
                go_up = go_up + 1;
            }

            println!("greates common divisor :{}, least common multiple : {}", div_concur, go_up);
            break;

        }
    }


}
