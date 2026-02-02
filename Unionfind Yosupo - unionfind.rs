use std::{io::{Read, stdin}, mem::swap};
struct DSU{
    parent: Vec<usize>,
    size: Vec<usize>
}
impl DSU{
    fn new(n:usize)->Self{
        Self {
            parent:(0..=n).collect(),
            size: vec![1;n+1],
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
    }
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut q:usize = it.next().unwrap().parse().unwrap();
    let mut dsu = DSU::new(n+1);
    while q!=0{
        let o:usize = it.next().unwrap().parse().unwrap();
        let u:usize = it.next().unwrap().parse().unwrap();
        let v:usize = it.next().unwrap().parse().unwrap();
        if o==0{
            dsu.union(u, v);
        }else{
            println!("{}",if dsu.find(u)==dsu.find(v) {1} else {0})
        }
        q-=1;
    }
}