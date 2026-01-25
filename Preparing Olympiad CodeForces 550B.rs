use std::{io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let l: i32 = it.next().unwrap().parse().unwrap();
    let r: i32 = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();
    let mut data = vec![0;n];
    let mut i=0;
    while i < n {
        data[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    i = 0;
    let limit = 1<<n;
    let mut j;
    let mut mn;
    let mut mx;
    let mut sum;
    let mut ans = 0;
    let mut c;
    while i<limit{
        j = 0;
        sum=0;
        c=0;
        mn = i32::MAX;
        mx = i32::MIN;
        while j<n{
            if i&(1<<j) != 0{
                c+=1;
                mn = mn.min(data[j]);
                mx = mx.max(data[j]);
                sum+=data[j];
            }
            j+=1;
        }
        ans += if c>1 && l<=sum && sum<=r && mx-mn>=x {1} else {0};
        i+=1;
    }
    println!("{ans}");
}