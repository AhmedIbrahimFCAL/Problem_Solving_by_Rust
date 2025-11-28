use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let k:usize = it.next().unwrap().parse().unwrap();
    let q:usize = it.next().unwrap().parse().unwrap();
    let N = 200002usize;
    let mut temps = vec![0i32;N];
    let mut i = 0;
    while i<n{
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        temps[a]+=1;
        temps[b+1]-=1;
        i+=1;
    }
    i=1;
    let mut sum = 0;
    while i<N {
        sum+=temps[i];
        temps[i] = if sum<k as i32 {0} else {1};
        temps[i] += temps[i-1];
        i+=1;
    }
    i=0;
    while i<q{
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        println!("{}",temps[b]-temps[a-1]);
        i+=1;
    }
}