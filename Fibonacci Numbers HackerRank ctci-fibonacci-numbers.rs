use std::io::stdin;

fn fib(n:usize)->usize{
    if n<3{return 1;}
    fib(n-1)+fib(n-2)
}
fn main(){
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("");
    print!("{}",fib(buf.trim().parse().unwrap()));
}