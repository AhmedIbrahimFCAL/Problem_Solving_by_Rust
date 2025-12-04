use std::{io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut n:u32 = it.next().unwrap().parse().unwrap();
    let k:usize = it.next().unwrap().parse().unwrap();
    let mut factors = Vec::<u32>::new();
    let sqn = (n as f32).sqrt().round() as u32;
    let mut i =2;
    if k==1{
        println!("{n}");
        return;
    }
    'fact: while i<=sqn{
        while n%i==0 && n!=1{
            factors.push(i);
            n/=i;
            if n==1{
                break 'fact;
            }
            if factors.len()==k-1{
                factors.push(n);
                break 'fact;
            }
        }
        i+=1;
    }
    if factors.len()!=k{
        println!("-1");
    }
    else {
        println!("{}",factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }

}