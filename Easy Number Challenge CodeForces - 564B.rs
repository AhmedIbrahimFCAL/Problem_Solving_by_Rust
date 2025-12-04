use std::{io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let a:usize = it.next().unwrap().parse().unwrap();
    let b:usize = it.next().unwrap().parse().unwrap();
    let c:usize = it.next().unwrap().parse().unwrap();
    let n = a*b*c +1;
    let mut cntDiv = vec![0;n];
    let mut i = 1;
    let mut j;
    while i<n{
        j = i;
        while j<n{
            cntDiv[j]+=1;
            j+=i;
        }
        i+=1;
    }
    i=1;
    let mut k;
    let mut ans = 0;
    while i<=a{
        j=1;
        while j<=b{
            k=1;
            while k<=c{
                ans = (ans+cntDiv[i*j*k])%1073741824;
                k+=1;
            }
            j+=1;
        }
        i+=1;
    }
    println!("{ans}");
}