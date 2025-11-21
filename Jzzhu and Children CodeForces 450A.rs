use std::{collections::VecDeque, io::stdin};
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    let m:u8 = input.split_whitespace().last().unwrap().parse().unwrap();
    input.clear();
    stdin().read_line(&mut input).expect("");
    let mut q:VecDeque<(usize,u8)> = input.split_whitespace().map(|x| x.parse::<u8>().unwrap()).enumerate().collect();
    loop{
        let n = q.pop_front().unwrap();
        if n.1 > m{
            q.push_back((n.0,n.1-m));
        }
        if q.is_empty(){
            println!("{}",n.0 +1);
            break;
        }
    }
}