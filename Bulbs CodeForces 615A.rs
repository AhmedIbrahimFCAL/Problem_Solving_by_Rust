use std::{collections::{HashMap, HashSet}, io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut bulbs = vec![false;m];
    for _ in 0..n{
        let w = it.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..w{
            bulbs[it.next().unwrap().parse::<usize>().unwrap()-1] = true;
        }
    }
    if bulbs.contains(&false){
        println!("NO");
    }else{
        println!("YES");
    }
}