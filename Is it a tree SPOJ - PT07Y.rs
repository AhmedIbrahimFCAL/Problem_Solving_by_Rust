use std::io::{Read, stdin};

fn dfs(adj:&Vec<Vec<usize>>,vis:&mut Vec<u8>, u:usize) -> usize{
    vis[u] = 1;
    let mut size = 1;
    for &neig in &adj[u]{
        if vis[neig] == 0 {
            size+=dfs(adj, vis, neig);
        }
    }
    size
}
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let m:usize = it.next().unwrap().parse().unwrap();
    let mut i =0;
    let mut adj = vec![Vec::<usize>::new();n];
    let mut vis = vec![0u8; n];
    while i<m{
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        adj[a-1].push(b-1);
        i+=1;
    }
    println!("{}", if n-1==m && dfs(&mut adj,&mut vis, 0)==n{"YES"} else {"NO"});
}
