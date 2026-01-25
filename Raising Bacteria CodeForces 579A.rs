use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    println!("{}",buf.trim().parse::<u32>().unwrap().count_ones());
}