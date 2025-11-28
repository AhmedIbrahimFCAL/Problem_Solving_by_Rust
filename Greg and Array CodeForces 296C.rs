use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let s:usize = it.next().unwrap().parse().unwrap();
    let q:usize = it.next().unwrap().parse().unwrap();
    let mut data = vec![0;n+1];
    let mut queries = vec![(0,0,0i64);s];
    let mut prequeries = vec![0;s+2];
    for i in 0..n{
        data[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 0..s{
        queries[i] = (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap()
        );
    }
    for _ in 0..q{
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        prequeries[a]+=1;
        prequeries[b+1]-=1;
    }
    for i in 1..prequeries.len(){
        prequeries[i]+=prequeries[i-1];
    }
    let mut data2 = vec![0;n+1];
    for i in 0..queries.len(){
        let t = &queries[i];
        data2[(t.0-1) as usize] += t.2 * prequeries[i+1]; 
        data2[t.1 as usize] -= t.2 * prequeries[i+1];
    }
    for i in 1..data2.len(){
        data2[i]+=data2[i-1];
    }
    for i in 0..n{
        print!("{} ", data[i]+data2[i]);
    }
}