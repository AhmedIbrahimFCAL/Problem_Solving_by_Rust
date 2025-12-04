use std::io::{Read, stdin};
fn is_hard_to_read(chars:&[u8], i:usize)->bool{
    if i==chars.len(){return false;}
    ((i&1==0) ^ (chars[i]>b'Z')) || is_hard_to_read(chars, i+1)
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let chars = buf.trim().as_bytes();
    println!("{}",if !is_hard_to_read(chars, 0) {"Yes"} else {"No"});
}