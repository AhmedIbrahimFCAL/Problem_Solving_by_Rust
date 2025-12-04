use std::{cmp::Ordering, io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut periods = vec![(0,0);n];
    let mut i = 0;
    while i<n{
        periods[i]=(
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap()
        );
        i+=1;
    }
    periods.sort_by(|a,b| 
        match a.1.cmp(&b.1){
            Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    );// this faster from the next
    // periods.sort_by_key(|x| (x.1,x.0));
    let mut last = -1;
    i=0;
    let mut ans = 0;
    while i<n{
        if periods[i].0>last{
            last = periods[i].1;
            ans+=1;
        }
        i+=1;
    }
    println!("{ans}");
}