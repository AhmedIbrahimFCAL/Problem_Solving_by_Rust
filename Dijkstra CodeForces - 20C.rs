use std::{cmp::Reverse, collections::BinaryHeap, io::{Read, stdin}};
fn dijkstra(adj:&Vec<Vec<(usize,usize)>>, dist:&mut Vec<(usize,usize)>, start:usize){
    let mut pq = BinaryHeap::new();
    dist[start] = (0,0);
    pq.push(Reverse((0,start)));
    while !pq.is_empty(){
        let Reverse((w,u)) = pq.pop().unwrap();
        if dist[u].0 != w {continue;}
        for &(v,d) in &adj[u]{
            if dist[v].0 > d+w{
                dist[v] = (d+w, u);
                pq.push(Reverse((d+w,v)));
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
    let mut i = 0;
    while i<m{
        let a = it.next().unwrap().parse::<usize>().unwrap()-1;
        let b = it.next().unwrap().parse::<usize>().unwrap()-1;
        let w = it.next().unwrap().parse::<usize>().unwrap();
        adj[a].push((b,w));
        adj[b].push((a,w));
        i+=1;
    }
    let mut dist = vec![(usize::MAX,usize::MAX);n];
    dijkstra(&adj, &mut dist, 0);
    if dist[n-1].0 != usize::MAX{
        let mut k = dist[n-1].1;
        let mut ans = Vec::new();
        while k!=0{
            ans.push(k+1);
            k = dist[k].1;
        }
        println!("1 {} {n}",ans.iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }else{
        println!("-1");
    }
}