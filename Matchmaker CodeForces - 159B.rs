use std::{collections::{HashMap}, io::{Read, stdin}};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let m:usize = it.next().unwrap().parse().unwrap();
    let mut markers = HashMap::<(u16, u16),u32>::new();
    let mut caps = Vec::new();
    let mut i=0;
    while i<n{
        *markers.entry((
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )).or_insert(0) += 1;
        i+=1;
    }
    let mut n_beautiful = 0;
    let mut n_matched = 0;
    i=0;
    while i<m{
        let cap = (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        );
        if let Some(count) = markers.get_mut(&cap){
            *count -= 1;
            if *count==0{
                markers.remove(&cap);
            }
            n_beautiful+=1;
            n_matched+=1;
        }else{
            caps.push(cap);
        }
        i+=1;
    }
    i=0;
    let mut residual_markers = HashMap::new();
    for i in markers{
        *residual_markers.entry(i.0.1).or_insert(0) += i.1;
    }
    // let mut markers:HashMap<u16,u32> = markers.iter().map(|(&(_,b),&c)| (b,c)).collect();
    while i<caps.len() {
        if let Some(count) = residual_markers.get_mut(&caps[i].1){
            n_matched+=1;
            *count -=1;
            if *count == 0{
                residual_markers.remove(&caps[i].1);
            }
        }
        i+=1;
    }    
    println!("{n_matched} {n_beautiful}");
}
