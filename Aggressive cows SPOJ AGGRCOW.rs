use std::io::{Read, stdin};

fn is_gressive_cow(stalls:&Vec<u32>, c:u32,dist:u32)->bool{
    let mut rem = c-1;
    let mut last = stalls[0];
    let mut i =1;
    while rem>0 && i<stalls.len(){
        if stalls[i]-last >=dist{
            last = stalls[i];
            rem-=1;
        }
        i+=1;
    }
    rem==0
}

fn bs(stalls:&Vec<u32>,c:u32,mut s:u32,mut e:u32)->u32{
    while e-s>1{
        let mid = (e+s)>>1;
        if is_gressive_cow(stalls, c, mid){
            s = mid;
        }else{
            e = mid;
        }
    }
    s
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut t:u32 = it.next().unwrap().parse().unwrap();
    while t>0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let c:u32 = it.next().unwrap().parse().unwrap();
        let mut stalls = vec![0;n as usize];
        let mut i:usize =0;
        while i<n{
            stalls[i] = it.next().unwrap().parse().unwrap();
            i+=1;
        }
        stalls.sort();
        println!("{}",bs(&stalls, c, 1, (stalls[n-1]-stalls[0])/(c-1)));
        t-=1;
    }
}