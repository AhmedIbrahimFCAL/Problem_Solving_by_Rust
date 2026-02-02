use std::io::{Read, stdin};

struct UpdatedPrefixArray{
    data: Vec<usize>,
}
impl UpdatedPrefixArray{
    fn new(n:usize)->Self{
        Self { data: vec![0;n] }
    }
    // note that if i is power of 2, the it contains all sums, because all number which less then it will go to it
    // i is one-based
    fn add(&mut self, mut i:usize, value:usize){
        while i <= self.data.len(){
            self.data[i-1] += value;
            i += i & (!i).wrapping_add(1)
        }
    }
    fn sub(&mut self, mut i:usize, value:usize){
        while i <= self.data.len(){
            self.data[i-1] -= value;
            i += i & (!i).wrapping_add(1)
        }
    }
    fn get_cumulative(&self, mut i:usize) -> usize{
        let mut sum = 0;
        while i != 0{
            sum += self.data[i-1];
            i -= i & (!i).wrapping_add(1);
        }
        sum
    }
    fn get_value(&self, i:usize) -> usize{
        self.get_range(i, i)
    }
    fn get_range(&self, start:usize, end:usize) -> usize{
        self.get_cumulative(end) - self.get_cumulative(start-1)
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut t:usize = it.next().unwrap().parse().unwrap();
    let mut s = 1;
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut q:usize = it.next().unwrap().parse().unwrap();
        let mut i = 1;
        let mut pref = UpdatedPrefixArray::new(n);
        while i <= n{
            pref.add(i, it.next().unwrap().parse().unwrap());
            i+=1;
        }
        println!("Case {s}:");
        s+=1;
        while q != 0{
            let a:u8 = it.next().unwrap().parse().unwrap();
            let i = it.next().unwrap().parse::<usize>().unwrap()+1;
            match a {
                1 => {
                    let x = pref.get_value(i);
                    println!("{}",x);
                    pref.sub(i, x);
                },
                2 => {
                    pref.add(i, it.next().unwrap().parse::<usize>().unwrap());
                },
                _ => {
                    println!("{}",pref.get_range(i, it.next().unwrap().parse::<usize>().unwrap()+1))
                }
            }
            q-=1;
        }
        t-=1;
    }
}