use std::io::stdin;
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.clear();
    stdin().read_line(&mut input).expect("");
    let data:Vec<u8> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut ones:Vec<u16> = Vec::new();
    let mut twos:Vec<u16> = Vec::new();
    let mut thrs:Vec<u16> = Vec::new();
    for i in 0..data.len(){
        if data[i] == 1{
            ones.push((i+1) as u16);
        }else if data[i] == 2{
            twos.push((i+1) as u16);
        }else{
            thrs.push((i+1) as u16);
        }
    }
    let n = ones.len().min(twos.len().min(thrs.len()));
    println!("{n}");
    for _ in 0..n{
        println!("{} {} {}", ones.pop().unwrap(), twos.pop().unwrap(), thrs.pop().unwrap());
    }
}