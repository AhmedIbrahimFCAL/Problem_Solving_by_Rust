use std::{collections::{HashSet}, io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<u8>().unwrap();
    for _ in 0..t{
        let mut set = HashSet::new();
        let n:u8 = it.next().unwrap().parse().unwrap();
        let m:u8 = it.next().unwrap().parse().unwrap();
        for _ in 0..n{
            set.insert(it.next().unwrap().parse::<u8>().unwrap());
        }
        let mut ans:u16 = 0;
        for _ in 0..m{
            if !set.insert(it.next().unwrap().parse::<u8>().unwrap()){
                ans+=1;
            }
        }
        println!("{ans}");
    }
}