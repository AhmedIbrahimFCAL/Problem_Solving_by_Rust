use std::{collections::HashSet, io::stdin};
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    let mut chars = [0;26];
    let mut chars = HashSet::new();
    for i in input.trim().as_bytes(){
        if 96<*i && *i<123{
            chars.insert(i);
        }
    }
    println!("{}",chars.len());
}
