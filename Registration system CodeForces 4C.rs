use std::{collections::{HashMap, HashSet}, io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<u32>().unwrap();
    let mut map = HashMap::new();
    for _ in 0..n{
        
        let txt = it.next().unwrap();
        match map.get_mut(txt){
            None=>{
                println!("OK");
                map.insert(txt, 0);
            },
            Some(num)=>{
                *num +=1 ;
                println!("{}{}",txt,*num);
            }
        }
    }
}