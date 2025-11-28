use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut prefix = vec![0;n+1];
    for i in 1..=n{
        prefix[i] = it.next().unwrap().parse().unwrap();
        prefix[i]+= prefix[i-1];
    }
    let q = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..q{
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        println!("{}",prefix[b+1]-prefix[a]);
    }
}