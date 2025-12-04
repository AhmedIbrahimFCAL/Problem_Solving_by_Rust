use std::io::{Read, stdin};
fn hide_xs(data:&mut Vec<Option<u32>>,x:u32,i:usize){
    if i==data.len(){return;}
    if data[i].unwrap() == x{
        data[i] = None;
    }
    hide_xs(data, x, i+1);
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let x:u32 = it.next().unwrap().parse().unwrap();
    let mut data = vec![None;n];
    let mut i = 0;
    while i<n{
        data[i] = Some(it.next().unwrap().parse::<u32>().unwrap());
        i+=1;
    }
    hide_xs(&mut data, x, 0);
    println!("{}",data.iter().map(|&x| if x==None {String::from("")} else{x.unwrap().to_string()}).collect::<Vec<_>>().join(" "));
}

/* use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:u32 = it.next().unwrap().parse().unwrap();
    let x:u32 = it.next().unwrap().parse().unwrap();
    let mut i = 0;
    while i<n{
        let a = it.next().unwrap().parse::<u32>().unwrap();
        if a != x{
            print!("{a} ");
        }    
        i+=1;
    }
} */