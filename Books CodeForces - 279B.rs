use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(& mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<usize>().unwrap();
    let mut data = vec![0;n];
    let mut mx = 0;
    let mut i =0;
    let mut j =0;
    while i<n{
        data[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    i=0;
    let mut sum = 0;
    while j<n{
        sum+=data[j];
        if sum<=k{
            mx = mx.max(j-i+1);
        }else{
            sum-=data[i];
            i+=1;
        }
        j+=1;
    }
    println!("{mx}");
}