use std::io::{Read, stdin};
fn main(){
    let N:usize = 2e5 as usize + 2;
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    let mut preSum = vec![0;N];
    let mut preMn = vec![0;N];
    let mut preMx = vec![0;N];
    let mut sufSum = vec![0;N];
    let mut sufMn = vec![0;N];
    let mut sufMx = vec![0;N];
    for _ in 0..t{
        let n = it.next().unwrap().parse::<usize>().unwrap();
        let q = it.next().unwrap().parse::<usize>().unwrap();
        let txt = it.next().unwrap().as_bytes();
        for i in 1..=n{
            preSum[i] = preSum[i-1] + if txt[i-1] == b'+' {1} else {-1};
            preMn[i] = preMn[i-1].min(preSum[i]);
            preMx[i] = preMx[i-1].max(preSum[i]);
        }
        sufMn[n]=0;sufSum[n]=0;sufMx[n]=0;
        sufMn[n+1]=0;sufSum[n+1]=0;sufMx[n+1]=0;
        for i in (1..=txt.len()).rev(){
            sufSum[i] = sufSum[i+1] + if txt[i-1]==b'-' {1} else {-1};
            if txt[i-1] == '-' as u8{
                sufSum[i] = sufSum[i+1] +1;
            }else{
                sufSum[i] = sufSum[i+1] -1;
            }
            sufMn[i] = sufMn[i+1].min(sufSum[i]);
            sufMx[i] = sufMx[i+1].max(sufSum[i]);
        }
        for _ in 0..q{
            let s = it.next().unwrap().parse::<usize>().unwrap() -1;
            let e = it.next().unwrap().parse::<usize>().unwrap() +1;
            let mut mn = preMn[s];
            let mut mx = preMx[s];
            let delta = preSum[s]-sufSum[e];
            mn = mn.min(delta+sufMn[e]);
            mx = mx.max(delta+sufMx[e]);
            println!("{}",mx-mn+1);
        }
    }
}


/*
use std::io::{Read, stdin};
fn main(){
    let N:usize = 2e5 as usize + 2;
    let mut preSum = vec![0;N];
    let mut preMn = vec![0;N];
    let mut preMx = vec![0;N];
    let mut sufSum = vec![0;N];
    let mut sufMn = vec![0;N];
    let mut sufMx = vec![0;N];
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t{
        let n = it.next().unwrap().parse::<usize>().unwrap();
        let q = it.next().unwrap().parse::<usize>().unwrap();
        let txt = it.next().unwrap().as_bytes();
        for i in 1..=n{
            if txt[i-1] == '+' as u8{
                preSum[i] = preSum[i-1] +1;
            }else{
                preSum[i] = preSum[i-1] -1;
            }
            preMn[i] = preMn[i-1].min(preSum[i]);
            preMx[i] = preMx[i-1].max(preSum[i]);
        }
        sufMn[n]=0;sufSum[n]=0;sufMx[n]=0;
        for i in (1..=txt.len()).rev(){
            if txt[i-1] == '-' as u8{
                sufSum[i] = sufSum[i+1] +1;
            }else{
                sufSum[i] = sufSum[i+1] -1;
            }
            sufMn[i] = sufMn[i+1].min(sufSum[i]);
            sufMx[i] = sufMx[i+1].max(sufSum[i]);
        }
        for _ in 0..q{
            let s = it.next().unwrap().parse::<usize>().unwrap() -1;
            let e = it.next().unwrap().parse::<usize>().unwrap() +1;
            // println!("{}, {}",s,e);
            let mut mn = preMn[s];
            let mut mx = preMx[s];
            // println!("{mn}->{mx}  {}",preSum[s]);
            // println!("{}->{}  {}",sufMn[e],sufMx[e],sufSum[e]);
            let delta = preSum[s]-sufSum[e];
            mn = mn.min(delta+sufMn[e]);
            mx = mx.max(delta+sufMx[e]);
            // println!("^{delta}, {mx}->{mn}");
            println!("{}",mx-mn+1);
        }
        // println!("PreSm: {:?}",&preSum[0..=n]);
        // println!("PreMx: {:?}",&preMx[0..=n]);
        // println!("PreMn: {:?}",&preMn[0..=n]);
        // println!("");
        // println!("SufSm: {:?}",&sufSum[0..=n]);
        // println!("SufMx: {:?}",&sufMx[0..=n]);
        // println!("SufMn: {:?}",&sufMn[0..=n]);
        // for i in 1..=n{
        //     println!("{}")
        // }
        // println!("PreSm: {:?}",preSum);
        // println!("PreMX: {:?}",preMx);
        // println!("PreMn: {:?}",preMn);
        // println!("");
        // println!("SufSm: {:?}",sufSum);
        // println!("SufMX: {:?}",sufMx);
        // println!("SufMn: {:?}",sufMn);
    }
}


2
8 4
-+--+--+
1 8
2 8
2 5
1 1
4 10
+-++
1 1
1 2
2 2
1 3
2 3
3 3
1 4
2 4
3 4
4 4

*/

/* // Naive Solution
use std::io::{Read, stdin};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let t = it.next().unwrap().parse::<u16>().unwrap();
    for _ in 0..t{
        let n = it.next().unwrap().parse::<u32>().unwrap();
        let q = it.next().unwrap().parse::<u32>().unwrap();
        let txt = it.next().unwrap().as_bytes();
        for _ in 0..q{
            let a = it.next().unwrap().parse::<usize>().unwrap();
            let b = it.next().unwrap().parse::<usize>().unwrap();
            let mut x = 0;
            let mut mx = 0;
            let mut mn = 0;
            for i in 1..=txt.len(){
                if a<=i && i<=b {continue;}
                if txt[i-1] == '+' as u8{
                    x+=1;
                }else{
                    x-=1;
                }
                mn = mn.min(x);
                mx = mx.max(x);
            }
            println!("{}",mx-mn+1);
        }
    }
} */