use std::io::{Read, stdin};
struct BIT<T>{
    data: Vec<T>,
    default: T,
    operation: fn(T,T)->T,
    operation_inv: fn(T,T)->T,
}
impl<T:Copy> BIT<T>{
    fn new(n:usize, default:T,operation:fn(T,T)->T, operation_inv:fn(T,T)->T)->Self{
        Self { data: vec![default;n], default, operation, operation_inv}
    }
    // note that if i is power of 2, the it contains all sums, because all number which less then it will go to it
    // i is one-based
    fn operate(&mut self, mut i:usize, value:T){
        while i <= self.data.len(){
            self.data[i-1] = (self.operation)(self.data[i-1],value);
            i += i & i.wrapping_neg()
        }
    }
    fn operate_inv(&mut self, mut i:usize, value:T){
        while i <= self.data.len(){
            self.data[i-1] = (self.operation_inv)(self.data[i-1],value);
            i += i & i.wrapping_neg()
        }
    }
    fn get_cumulative(&self, mut i:usize) -> T{
        let mut sum = self.default;
        while i != 0{
            sum = (self.operation)(sum, self.data[i-1]);
            i -= i & i.wrapping_neg()
        }
        sum
    }
    fn get_value(&self, i:usize) -> T{
        self.get_range(i, i)
    }
    fn get_range(&self, start:usize, end:usize) -> T{
        (self.operation_inv)(self.get_cumulative(end), self.get_cumulative(start-1))
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut q:usize = it.next().unwrap().parse().unwrap();
    let mut i = 0;
    let mut pref = BIT::<usize>::new(n,0,|x,y| x+y, |x,y| x.wrapping_add(y.wrapping_neg()));
    let mut data = vec![0;n];
    while i < n{
        data[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    while q != 0{
        let t:u8 = it.next().unwrap().parse().unwrap();
        let a = it.next().unwrap().parse::<usize>().unwrap();
        match t {
            1 => {
                let b:usize = it.next().unwrap().parse().unwrap();
                let v:usize = it.next().unwrap().parse().unwrap();
                pref.operate(a, v);
                pref.operate_inv(b+1, v);
            },
            _ => {
                println!("{}",pref.get_cumulative(a) + data[a-1]);
            },
        }
        q-=1;
    }
}


/* optimized version by space complexity
use std::io::{Read, stdin};
struct BIT<T>{
    data: Vec<T>,
    default: T,
    operation: fn(T,T)->T,
    operation_inv: fn(T,T)->T,
}
impl<T:Copy> BIT<T>{
    fn new(n:usize, default:T,operation:fn(T,T)->T, operation_inv:fn(T,T)->T)->Self{
        Self { data: vec![default;n], default, operation, operation_inv}
    }
    // note that if i is power of 2, the it contains all sums, because all number which less then it will go to it
    // i is one-based
    fn operate(&mut self, mut i:usize, value:T){
        while i <= self.data.len(){
            self.data[i-1] = (self.operation)(self.data[i-1],value);
            i += i & i.wrapping_neg()
        }
    }
    fn operate_inv(&mut self, mut i:usize, value:T){
        while i <= self.data.len(){
            self.data[i-1] = (self.operation_inv)(self.data[i-1],value);
            i += i & i.wrapping_neg()
        }
    }
    fn get_cumulative(&self, mut i:usize) -> T{
        let mut sum = self.default;
        while i != 0{
            sum = (self.operation)(sum, self.data[i-1]);
            i -= i & i.wrapping_neg()
        }
        sum
    }
    fn get_value(&self, i:usize) -> T{
        self.get_range(i, i)
    }
    fn get_range(&self, start:usize, end:usize) -> T{
        (self.operation_inv)(self.get_cumulative(end), self.get_cumulative(start-1))
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut q:usize = it.next().unwrap().parse().unwrap();
    let mut pref = BIT::<usize>::new(n,0,|x,y| x.wrapping_add(y), |x,y| x.wrapping_add(y.wrapping_neg()));
    let mut curr;
    let mut prev = 0;
    let mut i = 1;
    while i <= n{
        curr = it.next().unwrap().parse::<usize>().unwrap();
        pref.operate(i, curr.wrapping_sub(prev));
        prev = curr;
        i+=1;
    }
    while q != 0{
        let t:u8 = it.next().unwrap().parse().unwrap();
        let a = it.next().unwrap().parse::<usize>().unwrap();
        match t {
            1 => {
                let b:usize = it.next().unwrap().parse().unwrap();
                let v:usize = it.next().unwrap().parse().unwrap();
                pref.operate(a, v);
                pref.operate_inv(b+1, v);
            },
            _ => {
                println!("{}",pref.get_cumulative(a));
            },
        }
        q-=1;
    }
}
*/