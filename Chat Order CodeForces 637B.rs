use std::{collections::{HashSet}, io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut it = it.rev();
    let mut set = HashSet::new();
    for _ in 0..n{
        let txt = it.next().unwrap();
        
        if !set.contains(txt){
            println!("{txt}");
            set.insert(txt);
        }
    }
}