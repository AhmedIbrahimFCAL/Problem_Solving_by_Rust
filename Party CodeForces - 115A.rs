use std::{collections::VecDeque, io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut in_deg = vec![0;n];
    let mut adj = vec![Vec::<usize>::new(); n];
    let mut i = 0;
    let mut queue = VecDeque::<usize>::new();
    while i < n {
        let a = it.next().unwrap().parse::<i32>().unwrap()-1;
        if a > -1{
            let a = a as usize;
            in_deg[i] += 1;
            adj[a].push(i);
        }else{
            queue.push_back(i);
        }
        i+=1;
    }
    let mut ans = 0;
    let mut size;
    while !queue.is_empty(){
        ans+=1;
        size = queue.len();
        while size > 0{
            let u = queue.pop_front().unwrap();
            for &v in &adj[u]{
                queue.push_back(v);
            }
            size-=1;
        }
    }
    println!("{ans}");

}