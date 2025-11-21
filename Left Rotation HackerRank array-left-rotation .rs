use std::{collections::VecDeque, io::stdin};
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    let data:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let m = data[1];
    input.clear();
    stdin().read_line(&mut input).expect("");
    let mut dq:VecDeque<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for _ in 0..m{
        let x = dq.pop_front().unwrap();
        dq.push_back(x);
    }
    println!("{}",dq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
