use std::{io::{Read, stdin}, usize};

fn dfs(adj:&Vec<Vec<usize>>, trgt:&Vec<bool>, init:&Vec<bool>,ans:&mut Vec<usize>, u:usize, p:usize, mut odd:bool, mut even:bool, is_odd:bool){
    let cur = if is_odd {&mut odd} else {&mut even};
    if (init[u]^*cur)!=trgt[u]{
        ans.push(u+1);
        *cur = !*cur;
    }
    for &v in &adj[u]{
        if v!=p {
            dfs(adj, trgt, init, ans, v, u, odd, even, !is_odd);
        }
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut adj = vec![Vec::<usize>::new(); n];
    let mut ans = Vec::<usize>::new();
    let mut init = vec![false;n];
    let mut trgt = vec![false;n];
    let mut i=1;
    while i < n {
        let a = it.next().unwrap().parse::<usize>().unwrap()-1;
        let b = it.next().unwrap().parse::<usize>().unwrap()-1;
        adj[a].push(b);
        adj[b].push(a);
        i+=1;
    }
    i=0;
    while i<n{
        init[i] = it.next().unwrap() == "1";
        i+=1;
    }
    i=0;
    while i<n{
        trgt[i] = it.next().unwrap() == "1";
        i+=1;
    }
    dfs(&adj, &trgt, &init, &mut ans, 0, usize::MAX, false, false, false);
    i=0;
    println!("{}",ans.len());
    while i<ans.len(){
        println!("{}",ans[i]);
        i+=1;
    }
}