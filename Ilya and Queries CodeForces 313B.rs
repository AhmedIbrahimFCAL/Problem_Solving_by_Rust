use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let txt = it.next().unwrap().as_bytes();
    let mut prefix = vec![0;txt.len()+1];
    for i in 2..=txt.len(){
        prefix[i] = prefix[i-1] + if txt[i-1] == txt[i-2]{1}else{0};
    }
    let q = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..q{
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        println!("{}",prefix[b]-prefix[a]);
    }
}