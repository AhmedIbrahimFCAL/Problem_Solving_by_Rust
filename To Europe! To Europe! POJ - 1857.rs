use std::io::{Read, stdin};
fn to_europe(data:&Vec<(usize,f32)>, dp:&mut Vec<f32>,cap:usize, dst:f32, i:usize)->f32{
    if i>=data.len(){return 0.0}
    if dp[i]==dp[i]{return  dp[i]}
    let mut sum = 0;
    let mut mn = f32::MAX;
    let mut k = i;
    while k < data.len(){
        sum+=data[k].0;
        if sum > cap{break;}
        mn = mn.min(data[k].1);
        dp[i] = dp[i].min(dst/mn + to_europe(data, dp, cap, dst, k+1));
        k+=1;
    }
    dp[i]
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    loop{
        let cap:usize = it.next().unwrap().parse().unwrap();
        let dst:f32 = it.next().unwrap().parse().unwrap();
        let n:usize = it.next().unwrap().parse().unwrap();
        if cap==0 && dst==0.0 && n==0 {break}
        let mut data = vec![(0usize,0f32);n];
        let mut dp = vec![f32::NAN;n];
        let mut i=0;
        while i<n{
            data[i].0 = it.next().unwrap().parse().unwrap();
            data[i].1 = it.next().unwrap().parse().unwrap();
            i+=1;
        }
        println!("{:.1}",to_europe(&data, &mut dp, cap, dst, 0) * 60.0);
    }
}