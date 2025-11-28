use std::{collections::VecDeque, io::{Read, stdin}};
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<u32>().unwrap();
    let mut data:VecDeque<u8> = VecDeque::new();
    let mut reverse:bool = false;
    for _ in 0..n{
        let ch = it.next().unwrap().as_bytes()[0] as char;
        match ch{
            'b'|'f' => {
                if data.is_empty(){
                    println!("No job for Ada?");
                }else{
                    println!("{}", if (ch=='f') ^ reverse { data.pop_front().unwrap()} else{ data.pop_back().unwrap()});
                }
            },
            'r' => {
                reverse = !reverse;
            }
            _=>{
                let d = it.next().unwrap().parse::<u8>().unwrap();
                if (ch=='t') ^ reverse{
                    data.push_front(d);
                }else{
                    data.push_back(d);
                }
            }
        }
    }
}


/* use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // ---------- FAST INPUT ----------
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    // Read n
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut data: VecDeque<u8> = VecDeque::new();
    let mut reverse = false;

    for _ in 0..n {
        let op = it.next().unwrap();

        match op {
            "b" | "f" => {
                if data.is_empty() {
                    println!("No job for Ada?");
                } else {
                    let v = if (op == "f") ^ reverse {
                        data.pop_front().unwrap()
                    } else {
                        data.pop_back().unwrap()
                    };
                    println!("{}", v);
                }
            }

            "r" => {
                reverse = !reverse;
            }

            _ => {
                // op is "t" or something similar, next token is the number
                let val: u8 = it.next().unwrap().parse().unwrap();
                if (op == "t") ^ reverse {
                    data.push_front(val);
                } else {
                    data.push_back(val);
                }
            }
        }
    }
}*/