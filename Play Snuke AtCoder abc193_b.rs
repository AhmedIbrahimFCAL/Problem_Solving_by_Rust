use std::{io::{Read, stdin}, u32};
fn min_price(data:&Vec<(u32,u32,u32)>,i:usize)->u32{
    if i>=data.len(){return u32::MAX;}
    let me = if data[i].0 < data[i].2 {data[i].1} else {u32::MAX};
    me.min(min_price(data, i+1))
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut n:usize = it.next().unwrap().parse().unwrap();
    let mut data = vec![(0u32,0u32,0u32);n];
    while n!=0{
        let a:u32 = it.next().unwrap().parse().unwrap();
        let b:u32 = it.next().unwrap().parse().unwrap();
        let c:u32 = it.next().unwrap().parse().unwrap();
        data[n-1] = (a,b,c);
        n-=1;
    }
    let mn = min_price(&data, 0);
    println!("{}", if mn==u32::MAX {-1} else {mn as i32});
}

/* use std::{io::{Read, stdin}, u32};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut n:u32 = it.next().unwrap().parse().unwrap();
    let mut mn = u32::MAX;
    while n!=0{
        let a:u32 = it.next().unwrap().parse().unwrap();
        let b:u32 = it.next().unwrap().parse().unwrap();
        let c:u32 = it.next().unwrap().parse().unwrap();
        if a < c {
            mn = mn.min(b);
        }
        n-=1;
    }
    println!("{}", if mn==u32::MAX {-1} else {mn as i32});
} */