use std::{i32, io::{Read, stdin}};
fn GCD(mut a:i32,mut b:i32)->i32{
    let mut temp;
    while b!=0{
        temp = a;
        a = b;
        b = temp%b;
    }
    a
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.trim().split_whitespace();
    let mut t:usize = it.next().unwrap().parse().unwrap();
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut data = vec![0;n];
        let mut i = 0;
        let mut mn = i32::MAX;
        let mut mx = i32::MIN;
        while i<n{
            data[i] = it.next().unwrap().parse().unwrap();
            mn = mn.min(data[i]);
            mx = mx.max(data[i]);
            i+=1;
        }
        if mn == mx{
            println!("-1");
            t-=1;
            continue;
        }
        let mut gcd = 0;
        i = 0;
        while i<n{
            gcd = GCD(gcd, data[i]-mn);
            i+=1;
        }
        println!("{gcd}");
        t-=1;    
    }
}