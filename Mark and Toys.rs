use std::{io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let c:usize = it.next().unwrap().parse().unwrap();
    let mut arr = vec![0;n];
    let mut i =0;
    while i<n{
        arr[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    arr.sort();
    i=0;
    let mut ans = 0;
    let mut sum = 0;
    while i<n{
        sum+=arr[i];
        if sum>c{break;}
        ans+=1;
        i+=1;
    }
    println!("{ans}");
}


/* use std::{collections::HashMap, io::{Read, stdin}};
fn mark_and_toys(dp:&mut HashMap<(usize, usize),usize>,arr:&Vec<usize>, i:usize, residual:usize)->usize{
    if i==arr.len(){return 0;}
    if let Some(value) = dp.get(&(i,residual)){
        return *value
    }
    let ans = mark_and_toys(dp,arr, i+1, residual).max(
        if arr[i]<=residual {
            mark_and_toys(dp,arr, i+1, residual-arr[i]) + 1
        }else{
            0
        }
    );
    dp.insert((i,residual),ans);
    *dp.get(&(i,residual)).unwrap()
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let c:usize = it.next().unwrap().parse().unwrap();
    let mut dp  = HashMap::new();
    let mut arr = vec![0;n];
    let mut i =0;
    
    while i<n{
        arr[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    println!("{}",mark_and_toys(&mut dp, &arr, 0, c));
}
 */