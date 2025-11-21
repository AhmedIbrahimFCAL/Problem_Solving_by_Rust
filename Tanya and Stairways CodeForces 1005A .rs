use std::io::stdin;
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.clear();
    stdin().read_line(&mut input).expect("");
    let data:Vec<u16> = input.split_whitespace().map(|x| x.parse().expect("")).collect();
    let mut ind = 0;
    let mut ans:Vec<u16> = Vec::new();
    for i in 1..data.len(){
        if data[i] == 1{
            ans.push(i as u16 -ind);
            ind = i as u16;
        }
    }
    ans.push(data.len() as u16 - ind);
    println!("{}",ans.len());
    println!("{}",ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}