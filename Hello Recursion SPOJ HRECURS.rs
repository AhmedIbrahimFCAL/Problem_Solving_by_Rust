use std::io::{Read, stdin};
fn sum_rec(data:&mut Vec<i16>,i:usize)->i32{
    if i==data.len(){return 0;}
    data[i] as i32 + sum_rec(data, i+1)
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let t:u8 = it.next().unwrap().parse().unwrap();
    let mut i=0;
    while i<t{
        i+=1;
        let mut n:usize = it.next().unwrap().parse().unwrap();
        let mut data = vec![0i16;n];
        while n!=0{
            data[n-1] = it.next().unwrap().parse().unwrap();
            n-=1;
        }
        println!("Case {i}: {}",sum_rec(&mut data, 0));
    }
}