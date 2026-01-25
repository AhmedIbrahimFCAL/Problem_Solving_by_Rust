use std::{collections::VecDeque, io::{Read, stdin}};
fn bfs(adj:&mut Vec<Vec<usize>>, dist:&mut Vec<usize>, start:usize){ // Zero-Index
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty(){
        let u = queue.pop_front().unwrap();
        for &v in &adj[u]{
            if dist[v]==usize::MAX{
                dist[v] = u;
                queue.push_back(v);
            }
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
    let mut dist = vec![usize::MAX;n];
    let mut i =0;
    while i < m{
        let a = it.next().unwrap().parse::<usize>().unwrap()-1;
        let b = it.next().unwrap().parse::<usize>().unwrap()-1;
        adj[a].push(b);
        adj[b].push(a);
        i+=1;
    }
    bfs(&mut adj, &mut dist, 0);
    let mut ans = Vec::new();
    let mut k = dist[n-1];
    if k!=usize::MAX{
        while k!=0{
            ans.push(k+1);
            k = dist[k];
        }
        println!("{}",ans.len()+2);
        println!("1 {} {n}",ans.iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }else{
        println!("IMPOSSIBLE");
    }
}