use std::{io::{Read, stdin}, mem::swap};
struct DSU{
    parent: Vec<usize>,
    size: Vec<usize>,
    max:usize,
    count:usize,
}
impl DSU{
    fn new(n:usize)->Self{
        Self {
            parent:(0..=n).collect(),
            size: vec![1;n+1],
            max: 1,
            count: n
        }
    }
    fn find(&mut self,x:usize)->usize{
        if x == self.parent[x]{
            return x;
        }
        // path compression optimization
        self.parent[x] = self.find(self.parent[x]);  // back-track to minimize the height
        self.parent[x]
    }
    fn union(&mut self, mut x:usize, mut y:usize){
        x = self.find(x);
        y = self.find(y);
        if x==y {return;} // exactly unioned 
        if self.size[y] > self.size[x]{
            swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
        self.max = self.max.max(self.size[x]);
        self.count-=1;
    }
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut m:usize = it.next().unwrap().parse().unwrap();
    let mut dsu = DSU::new(n);
    while m!=0{
        let a:usize = it.next().unwrap().parse().unwrap();
        let b:usize = it.next().unwrap().parse().unwrap();
        dsu.union(a, b);
        println!("{} {}",dsu.count, dsu.max);        
        m-=1;
    }
}