use std::{io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut last_occurance = vec![0i32;200005];
    let mut t:usize = it.next().unwrap().parse().unwrap();
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut i = 0;
        while i<=n{
            last_occurance[i]=-1;
            i+=1;
        }
        let mut i=0i32;
        let n = n as i32;
        let mut mn = i32::MAX;
        while i<n{
            let k:usize = it.next().unwrap().parse().unwrap();
            if last_occurance[k]!=-1{
                mn = mn.min(i-last_occurance[k]+1);
            }
            last_occurance[k] = i;
            i+=1;
        }
        println!("{}",if mn==i32::MAX {-1} else {mn});
        t-=1;
    }
}