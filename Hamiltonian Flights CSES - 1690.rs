use std::{io::{Read, stdin}};

fn count_hamiltonian_flights(adj:&Vec<Vec<usize>>, dp:&mut Vec<Vec<usize>>, i:usize, mask:usize)->usize{
    let n = adj.len();
    if i == n-1{
        return if mask == (1<<n)-1 {1} else {0}
    }
    let mut ans = dp[i][mask];
    if ans!=usize::MAX{
        return ans
    }
    ans = 0;
    for &u in &adj[i]{
        if (1<<u) & mask == 0{
            ans += count_hamiltonian_flights(adj, dp, u, mask | (1<<u))
        }
    }
    ans %= 1000_000_007;
    dp[i][mask] = ans;
    ans
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut m:usize = it.next().unwrap().parse().unwrap();
    let mut adj = vec![Vec::<usize>::new();n];
    while m!=0{
        let a = it.next().unwrap().parse::<usize>().unwrap() -1;
        let b = it.next().unwrap().parse::<usize>().unwrap() -1;
        adj[a].push(b);
        m-=1;
    }
    let mut dp = vec![vec![usize::MAX;1<<n];n];
    println!("{}", count_hamiltonian_flights(&adj, &mut dp, 0, 1));
}