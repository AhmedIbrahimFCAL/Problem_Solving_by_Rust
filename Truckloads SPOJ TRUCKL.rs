use std::io::{Read, stdin};

fn calc_num_crates(n:usize,s:usize)->usize{
    if n<=s{return 1;}
    calc_num_crates(n>>1, s)+
    calc_num_crates((n>>1) + (n&1), s)
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    while let (Some(n), Some(s)) = (it.next(), it.next()){
        let n:usize = n.parse().unwrap();
        let s:usize = s.parse().unwrap();
        println!("{}",calc_num_crates(n, s));
    }
}