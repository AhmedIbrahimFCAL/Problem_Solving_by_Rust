use std::io::{Read, stdin};
struct BIT{
    m: Vec<isize>,
    c: Vec<isize>,
}
impl BIT{
    fn new(n:usize)->Self{
        Self { m: vec![0;n],c: vec![0;n]}
    }
    fn init(&mut self, n:usize){
        self.m[0..n].fill(0);
        self.c[0..n].fill(0);
    }
    // i is one-based
    fn add(&mut self, mut i:usize, ma:isize, ca:isize){
        while i <= self.m.len(){
            self.m[i-1] += ma;
            self.c[i-1] += ca;
            i += i & i.wrapping_neg()
        }
    }
    
    fn get_cumulative(&self, mut pos:usize) -> isize{
        let mut res = 0;
        let mut i = pos;
        let pos = pos as isize;
        while i != 0{
            res += pos * self.m[i-1] + self.c[i-1];
            i -= i & (!i).wrapping_add(1);
        }
        res
    }
    fn get_value(&self, i:usize) -> isize{
        self.get_range(i, i)
    }
    fn get_range(&self, start:usize, end:usize) -> isize{
        self.get_cumulative(end) - self.get_cumulative(start-1)
    }
    fn add_range(&mut self, start:usize, end:usize, value:isize){
        self.add(start, value, value*(1 - start as isize));
        self.add(end, -value, value * end as isize);
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let mut t:usize = it.next().unwrap().parse().unwrap();
    let mut bit = BIT::new(100_000);
    while t!=0{
        let n:usize = it.next().unwrap().parse().unwrap();
        let mut q:usize = it.next().unwrap().parse().unwrap();
        bit.init(n);
        while q!=0{
            let c:u8 = it.next().unwrap().parse().unwrap();
            let a:usize = it.next().unwrap().parse().unwrap();
            let b:usize = it.next().unwrap().parse().unwrap();
            match c {
                0 =>{
                    let v:isize = it.next().unwrap().parse().unwrap();
                    bit.add_range(a, b, v);
                },
                _ => {
                    println!("{}", bit.get_range(a, b));
                }
            }
            q-=1;
        }
        t-=1;
    }
}
