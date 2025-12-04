use std::io::{Read, stdin};
fn round_down(nums:&[u8],rounded_nums:&mut String, i:usize){
    if i==nums.len() || nums[i]==b'.'{return;}
    rounded_nums.push(nums[i] as char);
    round_down(nums, rounded_nums, i+1);
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut rounded_nums = String::new();
    let buf = buf.trim().as_bytes();
    round_down(buf, &mut rounded_nums, 0);
    println!("{}",rounded_nums);
}