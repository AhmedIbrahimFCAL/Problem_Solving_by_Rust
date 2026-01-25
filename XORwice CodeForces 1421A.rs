use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse().unwrap();
    let mut i =0;
    let mut a:u32;
    let mut b:u32;
    let mut c;
    while i<n{
        a = it.next().unwrap().parse().unwrap();
        b = it.next().unwrap().parse().unwrap();
        c = a&b;
        println!("{}",(a^c)+(b^c));
        i+=1;
    }
}