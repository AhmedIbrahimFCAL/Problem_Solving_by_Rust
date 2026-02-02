use std::{io::{Read, stdin}};
fn count_digit(dp:&mut Vec<Vec<Vec<Vec<usize>>>>,a:&Vec<u8>, b:&Vec<u8>, i:u8, last:u8, eq_a:bool, eq_b:bool)->usize{
    if i ==a.len() as u8{return 1}

    let mut ans = dp[i as usize][last as usize][eq_a as usize][eq_b as usize];
    if ans != usize::MAX{
        return ans
    }
    ans = 0;
    let lower_limit = if eq_a {a[i as usize] - b'0'} else {0};
    let upper_limit = if eq_b {b[i as usize] - b'0'} else {9};
    let mut d = lower_limit;
    while d <= upper_limit{
        if last==10 && d==0{
            ans += count_digit(dp, a, b, i+1, last, if d==lower_limit {eq_a} else{false}, if d==upper_limit {eq_b} else{false});
        }
        else if d != last{
            ans += count_digit(dp, a, b, i+1, d, if d==lower_limit {eq_a} else{false}, if d==upper_limit {eq_b} else{false});
        }
        d+=1;
    }
    dp[i as usize][last as usize][eq_a as usize][eq_b as usize] = ans;
    ans
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut a = it.next().unwrap().as_bytes().to_vec();
    let b = it.next().unwrap().as_bytes().to_vec();
    a.splice(0..0, vec![b'0';b.len()-a.len()]);
    let mut dp = vec![vec![vec![vec![usize::MAX;2];2];11];19];
    println!("{}",count_digit(&mut dp, &a, &b, 0, 10, true, true))
}