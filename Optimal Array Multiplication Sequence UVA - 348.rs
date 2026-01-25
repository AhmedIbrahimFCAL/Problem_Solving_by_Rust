use std::{io::{Read, stdin}};
// A1 A2 A3 A4 A5 A6 A7 A8 A9
fn optimal_array_multiplication_sequence(data:&Vec<(u32,u32)>,dp:&mut Vec<Vec<u32>>, s:usize,e:usize)->u32{
    if s>=e{return 0}
    if dp[s][e] != u32::MAX{return  dp[s][e]}
    let mut i = s;
    let mut res = u32::MAX;
    while i < e{
        res = res.min(
            optimal_array_multiplication_sequence(data, dp, s, i) + 
            optimal_array_multiplication_sequence(data, dp, i+1, e) +
            data[s].0 * data[i].1 * data[e].1
        );
        i+=1;
    }
    dp[s][e] = res;
    res
}
fn print(data:&Vec<(u32,u32)>,dp:&mut Vec<Vec<u32>>, s:usize,e:usize){
    if s==e{
        print!("A{}",s+1);
        return
    }
    let mut i = s;
    while i < e{
        if dp[s][e] == 
            optimal_array_multiplication_sequence(data, dp, s, i) +
            optimal_array_multiplication_sequence(data, dp, i+1, e) +
            data[s].0 * data[i].1 * data[e].1{
            print!("(");
            print(data, dp, s, i);
            print!(" x ");
            print(data, dp, i+1, e);
            print!(")");
            break
        }
        i+=1;
    }
} 
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut c = 0;
    loop{
        c+=1;
        let n:usize = it.next().unwrap().parse().unwrap();
        if n==0{break}
        let mut data = vec![(0u32,0u32);n];
        let mut dp = vec![vec![u32::MAX;n];n];
        let mut i=0;
        while i<n{
            data[i].0 = it.next().unwrap().parse().unwrap();
            data[i].1 = it.next().unwrap().parse().unwrap();
            i+=1;
        }
        optimal_array_multiplication_sequence(&data, &mut dp, 0, n-1);
        print!("Case {}: ",c); 
        print(&data, &mut dp, 0, n-1);
        println!();
    }
}
