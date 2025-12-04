// iterative solution
use std::{io::{Read, stdin}};
fn main(){
    let mut buf= String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let s:usize = it.next().unwrap().parse().unwrap();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut data = vec![(0,0);n];
    let mut dp = vec![vec![0u32;s+1];n+1];
    let mut i = n;
    while i>0{
        i-=1;
        data[i] = (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap());
    }
    let mut j;
    while i<=n{
        j=0;
        while j<=s{
            if i==0 || j==0{
                dp[i][j] = 0;
                j+=1;
                continue;
            }
            dp[i][j] = dp[i-1][j];
            if data[i-1].0<=j{
                dp[i][j]= dp[i][j].max(data[i-1].1+dp[i-1][j-data[i-1].0]);
            }
            j+=1;
        }
        i+=1;
    }
    println!("{}",dp[n][s]);
}

/* use std::{io::{Read, stdin}, vec};
fn calc_knapsack(data:&Vec<(i64,i64)>,dp:&mut Vec<Vec<i64>>, i:usize,s:i64)->i64{
    if i==data.len(){return 0;}
    if dp[i][s as usize]==-1{
        dp[i][s as usize] = calc_knapsack(data, dp, i+1, s);
        if data[i].0<=s{
            dp[i][s as usize] = dp[i][s as usize].max(calc_knapsack(data, dp, i+1, s-data[i].0)+data[i].1);
        }
    }
    dp[i][s as usize]
}
fn main(){
    let mut buf= String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let s:i64 = it.next().unwrap().parse().unwrap();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut data = vec![(0,0);n];
    let mut dp = vec![vec![-1;(s+1) as usize];n];
    let mut i = n;
    while i>0{
        i-=1;
        data[i] = (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap());
    }
    println!("{}",calc_knapsack(&data, &mut dp, 0, s));
} */