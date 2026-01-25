use std::{io::{Read, stdin}};
fn longest_increasing_subsequence(data:&Vec<usize>,dp:&mut Vec<usize>, l:usize)->usize{
    if dp[l] != usize::MAX{
        return dp[l];
    }
    let mut res = 1;
    let mut k = 0;
    while k < l{
        if data[k]<data[l]{
            res = res.max(longest_increasing_subsequence(data, dp, k)+1);
        }
        k+=1;
    }
    dp[l] = res;
    res
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize= it.next().unwrap().parse().unwrap();
    let data:Vec<usize> = it.take(n).map(|x| x.parse().unwrap()).collect();
    let mut dp = vec![usize::MAX;n+1];
    let mut res = 1;
    let mut i =0;
    while i<n {
        res = res.max(longest_increasing_subsequence(&data, &mut dp, i));
        i+=1;
    }
    println!("{}",res)
}

/* use std::{io::{Read, stdin}};
fn longest_increasing_subsequence(data:&Vec<usize>,dp:&mut Vec<Vec<usize>>, i:usize,l:usize)->usize{
    if i>=data.len(){return 0}
    if data[i]<=l{return 0} 
    if dp[i][l] != usize::MAX{
        return dp[i][l];
    }
    dp[i][l] = longest_increasing_subsequence(data,dp, i+1, l).max(
        longest_increasing_subsequence(data,dp, i+1, data[i])+1
    );
    dp[i][l]
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize= it.next().unwrap().parse().unwrap();
    let data:Vec<usize> = it.take(n).map(|x| x.parse().unwrap()).collect();
    let mut dp = vec![vec![usize::MAX; 20];n];
    println!("{}",longest_increasing_subsequence(&data,&mut dp, 0, 0))
} */