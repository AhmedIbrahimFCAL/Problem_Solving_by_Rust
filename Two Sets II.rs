use std::io::stdin;
fn two_sets(dp:&mut Vec<Vec<usize>>,n:usize,i:usize, sum:usize)->usize{
    if i==n {
        return (sum == n*(n+1)>>2) as usize
    }
    if dp[i][sum] != usize::MAX{
        return dp[i][sum];
    }
    dp[i][sum] = (two_sets(dp, n, i+1, sum) + two_sets(dp, n, i+1, sum+i)) % 1000_000_007;
    dp[i][sum]
}
fn main(){
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("");
    let n = buf.trim().parse::<usize>().unwrap();
    let mut dp = vec![vec![usize::MAX;(n*(n+1)>>1)+2];n+1];
    if (n*(n+1)>>1)&1 == 1{
        println!("0");
        return;
    }
    println!("{}",two_sets(&mut dp,n, 1, 0));
}