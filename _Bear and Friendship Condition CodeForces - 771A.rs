use std::{io::{Read, stdin}};
fn dfs(adj:&Vec<Vec<usize>>, vis:&mut Vec<bool>,c:&mut usize,e:&mut usize, u:usize){
    if u>=vis.len(){return}
    *c+=1;
    *e+=adj[u].len();
    vis[u] = true;
    for &v in &adj[u]{
        if !vis[v]{
            dfs(adj, vis,c,e, v);
        }
    }
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut adj = vec![Vec::new();n];
    let mut vis = vec![false;n];
    let mut i =0;
    while i<m{
        let a = it.next().unwrap().parse::<usize>().unwrap() -1;
        let b = it.next().unwrap().parse::<usize>().unwrap() -1;
        adj[a].push(b);
        adj[b].push(a);
        i+=1;
    }
    for i in 0..n{
        let mut e = 0;
        let mut c = 0;
        if !vis[i]{
            dfs(&adj, &mut vis, &mut c, &mut e, i);
            if c*(c-1)>>1 != e{
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}