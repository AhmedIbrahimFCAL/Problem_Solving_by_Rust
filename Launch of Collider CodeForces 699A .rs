use std::io:: stdin;
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.clear();
    let mut dirs = String::new();
    stdin().read_line(&mut dirs).expect("");
    stdin().read_line(&mut input).expect("");
    let data:Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut mn = u32::MAX;
    for i in 1..data.len(){
        if dirs.as_bytes()[i] == 'L' as u8 && dirs.as_bytes()[i-1] == 'R' as u8{
            mn = mn.min(data[i].abs_diff(data[i-1]));
        }
    }
    println!("{}", if mn==u32::MAX {-1} else {(mn >>1) as i32});
}