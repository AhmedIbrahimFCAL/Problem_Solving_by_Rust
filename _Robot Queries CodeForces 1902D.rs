use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let q:usize = it.next().unwrap().parse().unwrap();
    let dirs = it.next().unwrap().as_bytes();
    let mut predirs = vec![(0,0);n+1];
    let mut predirsMn = vec![(0,0);n+1];
    let mut predirsMx = vec![(0,0);n+1];
    let mut sufdirs = vec![(0,0);n+1];
    let mut sufdirsMn = vec![(0,0);n+1];
    let mut sufdirsMx = vec![(0,0);n+1];

    let mut predirs2 = vec![(0,0);n+1];
    let mut predirsMn2 = vec![(0,0);n+1];
    let mut predirsMx2 = vec![(0,0);n+1];
    let mut sufdirs2 = vec![(0,0);n+1];
    let mut sufdirsMn2 = vec![(0,0);n+1];
    let mut sufdirsMx2 = vec![(0,0);n+1];
    let mut i=0usize;
    while i<n{
        match dirs[i] {
            b'U' =>{predirs[i+1] = (predirs[i].0 ,predirs[i].1+1)}
            b'D' =>{predirs[i+1] = (predirs[i].0 ,predirs[i].1-1)}
            b'R' =>{predirs[i+1] = (predirs[i].0+1 ,predirs[i].1)}
            _ =>{predirs[i+1] = (predirs[i].0-1 ,predirs[i].1)}
        }
        predirsMx[i+1].0 = predirsMx[i].0.max(predirs[i+1].0);
        predirsMx[i+1].1 = predirsMx[i].1.max(predirs[i+1].1);
        predirsMn[i+1].0 = predirsMn[i].0.min(predirs[i+1].0);
        predirsMn[i+1].1 = predirsMn[i].1.min(predirs[i+1].1);
        i+=1;
    }
    while i>0 {
        match dirs[i-1] {
            b'U' =>{sufdirs[i-1] = (sufdirs[i].0 ,sufdirs[i].1+1)}
            b'D' =>{sufdirs[i-1] = (sufdirs[i].0 ,sufdirs[i].1-1)}
            b'R' =>{sufdirs[i-1] = (sufdirs[i].0+1 ,sufdirs[i].1)}
            _ =>{sufdirs[i-1] = (sufdirs[i].0-1 ,sufdirs[i].1)}
        }
        predirsMx[i-1].0 = predirsMx[i].0.max(predirs[i-1].0);
        predirsMx[i-1].1 = predirsMx[i].1.max(predirs[i-1].1);
        predirsMn[i-1].0 = predirsMn[i].0.min(predirs[i-1].0);
        predirsMn[i-1].1 = predirsMn[i].1.min(predirs[i-1].1);
        i-=1;
    }
    i=n;
    while i>0{
        match dirs[i] { // reverse operation
            b'U' =>{sufdirs2[i-1] = (sufdirs2[i].0 ,sufdirs2[i].1-1)}
            b'D' =>{sufdirs2[i-1] = (sufdirs2[i].0 ,sufdirs2[i].1+1)}
            b'R' =>{sufdirs2[i-1] = (sufdirs2[i].0-1 ,sufdirs2[i].1)}
            _    =>{sufdirs2[i-1] = (sufdirs2[i].0+1 ,sufdirs2[i].1)}
        }
        sufdirsMx2[i-1].0 = sufdirsMx2[i].0.max(sufdirs2[i-1].0);
        sufdirsMx2[i-1].1 = sufdirsMx2[i].1.max(sufdirs2[i-1].1);
        sufdirsMn2[i-1].0 = sufdirsMn2[i].0.min(sufdirs2[i-1].0);
        sufdirsMn2[i-1].1 = sufdirsMn2[i].1.min(sufdirs2[i-1].1);
        i-=1;
    }
    println!("{:?}",predirs);
    println!("{:?}",sufdirs);
    println!("{:?}",sufdirs2);
    it.next();
    it.next();
    // let a:usize = it.next().unwrap().parse().unwrap();
    let b:usize = it.next().unwrap().parse().unwrap();
    // let deltax = 
}