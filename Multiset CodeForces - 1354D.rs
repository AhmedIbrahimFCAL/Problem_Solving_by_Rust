use std::{io::{Read, stdin}};
struct MultiSet{
    data: Vec<u32>,
}
impl MultiSet{
    fn new(n:usize)->Self{
        let mut ms = Self { data: vec![0u32;n.next_power_of_two()]};
        ms.operate(1, u32::MAX);
        ms
    }
    // note that if i is power of 2, the it contains all sums, because all number which less then it will go to it
    // i is one-based
    fn operate(&mut self, mut i:usize, value:u32){
        while i <= self.data.len(){
            self.data[i-1] = self.data[i-1].wrapping_add(value);
            i += i & i.wrapping_neg()
        }
    }
    fn operate_inv(&mut self, mut i:usize, value:u32){
        while i <= self.data.len(){
            self.data[i-1] = self.data[i-1].wrapping_sub(value);
            i += i & i.wrapping_neg()
        }
    }
    fn get_cumulative(&self, mut i:usize) -> u32{
        let mut sum:u32 = 0;
        while i != 0{
            sum = sum.wrapping_add(self.data[i-1]);
            i -= i & (!i).wrapping_add(1);
        }
        sum
    }
    fn get_value(&self, i:usize) -> u32{
        self.get_range(i, i)
    }
    fn get_range(&self, start:usize, end:usize) -> u32{
        self.get_cumulative(end).wrapping_sub(self.get_cumulative(start-1))
    }
    
    fn insert(&mut self, value:usize, frequency:u32){
        self.operate(value, frequency);
    }
    fn erase(&mut self, value:usize, frequency:u32){
        self.operate(value, frequency.wrapping_neg());
    }
    fn erase_all(&mut self, value:usize){
        self.operate_inv(value, self.get_value(value));
    }
    fn count(&self, value:usize)->u32{
        self.get_cumulative(value)
    }
    fn size(&self)->u32{
        self.count(self.data.len()).wrapping_add(1)
    }
    fn lower_bound(&self, mut cummutative:u32)->usize{
        let mut start = 0;
        let mut size = self.data.len() >> 1;
        while size != 0{
            if (self.data[start + size -1] as i32) < cummutative as i32{
                start += size;
                cummutative = cummutative.wrapping_sub(self.data[start -1]);
            }
            size >>= 1;
        }
        start+1
    }
    fn print(&self){
        for i in 0..self.size(){
            print!("{}",self.lower_bound(i));
        }
        println!()
    }
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut q:usize = it.next().unwrap().parse().unwrap();
    let mut ms = MultiSet::new(n);
    let mut i = 0;
    while i < n{
        ms.insert(it.next().unwrap().parse().unwrap(), 1);
        i+=1;
    }
    let mut a:isize;
    while q!=0{
        a = it.next().unwrap().parse().unwrap();
        if a > 0{
            ms.insert(a as usize, 1);
        }else{ // a is -ve
            ms.erase(ms.lower_bound((-a -1) as u32) , 1);
        }
        q-=1;
    }
    if ms.size() == 0{
        println!("0");
    }else{
        println!("{}", ms.lower_bound(0));
    }
}