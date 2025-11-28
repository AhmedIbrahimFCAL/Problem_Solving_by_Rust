use std::{io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<u8>().unwrap();
    for _ in 0..n{
        it.next();
        let pat = it.next().unwrap();
        let mut sum = 0;
        let mut max_sum = 0;
        let mut is_A = false;
        for i in pat.as_bytes(){
            if !is_A && *i!=65{
                continue;
            }
            is_A=true;
            if *i == 65{sum=0;}
            else {sum+=1;}
            max_sum = max_sum.max(sum)
        }
        println!("{max_sum}");
    }
}