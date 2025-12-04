use std::io::{stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("");
    let n = buf.trim().parse::<usize>().unwrap();
    let mut primes = vec![0u8;n+1];
    let mut i = 2;
    while i<=n {
        if primes[i]==0{
            let mut j=i+i;
            while j<=n{
                if primes[j]<3{
                    primes[j]+=1;
                }
                j+=i;
            }
        }
        i+=1;
    }
    let mut sum = 0;
    i=2;
    while i<=n{
        if primes[i]==2{
            sum+=1;
        }
        i+=1;
    }
    println!("{sum}");
}