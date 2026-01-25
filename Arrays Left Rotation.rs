use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let d: usize = it.next().unwrap().parse().unwrap();
    let mut arr = vec![0;n];
    let mut i = 0;
    while i<n{
        arr[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    i = d;
    while i<n{
        print!("{} ",arr[i]);
        i+=1;
    }
    i = 0;
    while i<d{
        print!("{} ",arr[i]);
        i+=1;
    }
}