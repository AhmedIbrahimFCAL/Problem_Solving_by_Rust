use std::io::stdin;
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    let mut chars = [0;26];
    for i in input.trim().as_bytes(){
        chars[(i - 97) as usize] +=1;
    }
    let mut ans = 0;
    for i in 0..26{
        if chars[i]!=0{
            ans+=1;
        }
    }
    if ans&1==1{
        println!("IGNORE HIM!")
    }else{
        println!("CHAT WITH HER!")
    }
}
