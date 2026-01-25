use std::{collections::HashMap, io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut t:u32 = it.next().unwrap().parse().unwrap();
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut i =0;
        let mut freq = HashMap::new();
        while i<n{
            let a:u32 = it.next().unwrap().parse().unwrap();
            *freq.entry(a).or_insert(0) += 1;
            i+=1;
        }
        let mut mex = 0;
        loop{
            if let Some(_) = freq.get(&mex){
                mex+=1;
            }else{
                break;
            }
        }
        let mex = mex as usize;
        let mut dp = vec![u32::MAX;mex+1];
        i=0;
        while i<=mex{
            if i==0{
                dp[i]=0;
                i+=1;
                continue;
            }
            let mut j =0;
            while j<i{
                dp[i] = dp[i].min(dp[j]+i as u32 * (freq.get(&(j as u32)).unwrap()-1)+j as u32);
                j+=1; 
            }
            i+=1;
        }
        println!("{}",dp[mex]);
        t-=1;
    }
}

/* use std::{collections::HashMap, io::{Read, stdin}, u32};

fn calc(dp:&mut Vec<u32>,freq:&mut HashMap<u32,u32>, mex:u32)->u32{
    if mex==0{return 0;}
    if dp[mex as usize] != u32::MAX{
        return  dp[mex as usize];
    }
    let mut i =0;
    while i<mex{
        dp[mex as usize] = dp[mex as usize].min(calc(dp, freq, i) + mex*(freq.get(&i).unwrap()-1)+i);
        i+=1; 
    }
    dp[mex as usize]
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut t:u32 = it.next().unwrap().parse().unwrap();
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut i =0;
        let mut freq = HashMap::new();
        while i<n{
            let a:u32 = it.next().unwrap().parse().unwrap();
            *freq.entry(a).or_insert(0) += 1;
            i+=1;
        }
        let mut mex = 0;
        loop{
            if let Some(_) = freq.get(&mex){
                mex+=1;
            }else{
                break;
            }
        }
        let mut dp = vec![u32::MAX;mex as usize +1];
        println!("{}",calc(&mut dp, &mut freq, mex));

        t-=1;
    }
} */

/* 6
5 2 1 0 3 0 4 0
0: 3
1: 1
2: 1
3: 1
4: 1
5: 1
dp[i] = min m*(freq[i]-1)+i
*/