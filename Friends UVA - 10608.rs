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
    let mut t:i32 = it.next().unwrap().parse().unwrap();
    while t>0{   
        let n:usize = it.next().unwrap().parse().unwrap();
        let m:usize = it.next().unwrap().parse().unwrap();
        let mut i =0;
        let mut adj = vec![Vec::<usize>::new();n];
        let mut vis = vec![0u8; n];
        while i<m{
            let a:usize = it.next().unwrap().parse().unwrap();
            let b:usize = it.next().unwrap().parse().unwrap();
            adj[a-1].push(b-1);
            adj[b-1].push(a-1);
            i+=1;
        }
        let mut mx = 0;
        let mut k = 0;
        while k<n{
            if vis[k]==0{
                mx = mx.max(dfs(&mut adj,&mut vis, 0));
            }
            k+=1;
        }
        println!("{mx}");
        t-=1;
    }
}