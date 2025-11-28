use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap(); 
    let mut i=1;
    let mut data = vec![0u64;n+1];
    let mut data2 = vec![0u64;n+1];
    while i<=n{
        data[i] = it.next().unwrap().parse().unwrap();
        data2[i] = data[i];
        i+=1;
    }
    data2.sort();
    i=1;
    while i<=n{
        data[i]+=data[i-1];
        data2[i]+=data2[i-1];
        i+=1;
    }
    let q:usize = it.next().unwrap().parse().unwrap();
    i=0;
    while i<q{
        let u:u8 = it.next().unwrap().parse().unwrap();
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        if u==1{
            println!("{}",data[b] - data[a-1]);
        }else{
            println!("{}",data2[b] - data2[a-1]);
        }
        i+=1;
    }
}