use std::{io::{Read, stdin}};
fn dfs(adj:&Vec<Vec<usize>>, vis:&mut Vec<bool>,time:&mut Vec<(usize,usize)>,timer: &mut usize, u:usize){
    if u>=vis.len(){return}
    *timer += 1;
    vis[u] = true;
    time[u].0 = *timer;
    for &v in &adj[u]{
        if !vis[v]{
            dfs(adj, vis, time, timer, v);
        }
    }
    time[u].1 = *timer;
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut adj = vec![Vec::new(); n];
    let mut time = vec![(0usize,0usize);n];
    let mut vis = vec![false;n];
    let mut timer = 0usize;
    let mut i = 1;
    while i<n{
        let a = it.next().unwrap().parse::<usize>().unwrap()-1;
        adj[i].push(a);
        adj[a].push(i);
        i+=1;
    }
    dfs(&adj, &mut vis, &mut time, &mut timer, 0);
    
    println!("{}", time.iter().map(|x| (x.1 - x.0).to_string()).collect::<Vec<_>>().join(" "));
}

/* Timer Limit Solution

use std::{io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut data = vec![0;n];
    let mut ans = vec![0;n];
    let mut i = 1;
    let mut k ;
    while i<n{
        data[i] = it.next().unwrap().parse::<usize>().unwrap()-1;
        k = data[i];
        while k!=0{
            ans[k]+=1;
            k = data[k];
        }
        i+=1;
    }
    ans[0] = n-1;
    println!("{}",ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
} */