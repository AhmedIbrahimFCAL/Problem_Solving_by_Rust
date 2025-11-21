use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input = String::from("");
    stdin().read_line(&mut input).expect("");
    let data:Vec<i16> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut negs:Vec<i16> = Vec::new();
    let mut zeros:Vec<i16> = Vec::new();
    let mut poses:Vec<i16> = Vec::new();
    for i in 0..data.len(){
        if data[i] == 0{
            zeros.push(0);
        }else if data[i] < 0{
            negs.push(data[i]);
        }else{
            poses.push(data[i]);
        }
    }
    if poses.len()==0 {
        poses.push(negs.pop().unwrap());
        poses.push(negs.pop().unwrap());
    }
    if negs.len()&1 == 0 && negs.len()>0{ 
        zeros.push(negs.pop().unwrap());
    }
    println!("{} {}",negs.len(), negs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{} {}",poses.len(), poses.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{} {}",zeros.len(), zeros.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
