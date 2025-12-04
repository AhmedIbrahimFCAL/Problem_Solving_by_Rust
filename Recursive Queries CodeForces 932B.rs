use std::{io::{Read, stdin}};
fn f_product_non_zeros(mut n:u32)->u32{
    let mut res = 1;
    while n!=0{
        let a = n%10;
        if a!=0{
            res*=a;
        }
        n/=10;
    }
    res
}
fn g(data:&mut Vec<u32>, n:u32)->u32{
    if n<10{return n;}
    if data[n as usize] != u32::MAX{
        return data[n as usize];
    }
    data[n as usize] = g(data,f_product_non_zeros(n));
    data[n as usize]
}
fn main(){
    let N = 1000001;
    let mut data = vec![u32::MAX;N];
    let mut pre = vec![vec![0;10];N];
    let mut i=0;
    let mut count = 0;
    while i <N{
        data[i] = g(&mut data, i as u32);
        pre[i][data[i] as usize] = 1;
        i+=1;
    }
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut q:u32 = it.next().unwrap().parse().unwrap();
    i = 1;
    let mut j;
    while i<N{
        j=0;
        while j<10{
            pre[i][j] += pre[i-1][j];
            j+=1;
        }
        i+=1;
    }
    while q!=0{
        let l:usize = it.next().unwrap().parse().unwrap();
        let r:usize = it.next().unwrap().parse().unwrap();
        let x:usize = it.next().unwrap().parse().unwrap();
        println!("{}",pre[r][x]-pre[l-1][x]);
        q-=1;
    }
}