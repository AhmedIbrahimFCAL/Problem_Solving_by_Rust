use std::io::{Read, stdin};
fn sum_digits(data:&Vec<u8>, i:usize)->usize{
    if i==data.len(){return 0;}
    data[i] as usize + sum_digits(data, i+1)
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let data:Vec<u8> = it.next().unwrap().bytes().map(|x| x-b'0').collect();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut ans = (sum_digits(&data, 0)*n).to_string();
    while ans.len()!=1{
        let data:Vec<u8>  = ans.bytes().map(|x| x-b'0').collect();
        ans = sum_digits(&data, 0).to_string();
    }
    println!("{ans}");
}