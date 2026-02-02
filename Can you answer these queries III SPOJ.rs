use std::io::{Read, stdin};
// children i: 2i+1 (left), 2i+2 (right)
// parent i: (i-1)/2
#[derive(Clone)]
struct Entry{
    prefix: isize,
    sufix: isize,
    sum: isize,
    max:isize,
}
impl Entry{
    fn new(prefix:isize, sufix:isize, sum:isize, max:isize)->Self{
        Self { prefix, sufix, sum, max }
    }
}
struct SegmentTree{
    nodes:Vec<Entry>,
    size:usize,
}
impl SegmentTree{
    fn new(n:usize) -> Self{
        Self{nodes: vec![Entry::new(0,0,0,0);(n<<1).next_power_of_two()], size:n}
    }
    // node_index: index for tree, storing min at original data from node_start to node_end
    fn build(&mut self, data:&Vec<isize>){
        self._build(data,0, 0, self.len()-1);
    }
    fn len(&self)->usize{
        self.size
    }
    fn merge(a:&Entry,b:&Entry)->Entry{
        Entry::new(
            a.prefix.max(a.sum+b.prefix),
            b.sufix.max(b.sum + a.sufix),
            a.sum+b.sum,
            a.max.max(b.max.max(a.sufix + b.prefix))
        )
    }
    fn _build(&mut self,data:&Vec<isize>, node_index:usize, node_start:usize, node_end:usize){
        if node_start == node_end{
                  self.nodes[node_index] = Entry::new(data[node_start], data[node_start], data[node_start], data[node_start]);
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._build(data,left_child, node_start, mid);
        self._build(data,right_child, mid+1,node_end);
        self.nodes[node_index] = SegmentTree::merge(&self.nodes[left_child], &self.nodes[right_child])
    }
    fn update(&mut self,position:usize, value:isize){
        self._update(position, value, 0, 0, self.len()-1);
    }
    fn _update(&mut self,position:usize, value:isize, node_index:usize, node_start:usize, node_end:usize){
        if position < node_start || node_end < position {return}
        if node_start == node_end{
            self.nodes[node_index] = Entry::new(value, value, value, value);
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._update(position, value,left_child, node_start, mid);
        self._update(position, value,right_child, mid+1,node_end);
        self.nodes[node_index] = SegmentTree::merge(&self.nodes[left_child], &self.nodes[right_child])
    }
    fn query(&self, query_start:usize, query_end:usize)->Entry{
        self._query(query_start, query_end, 0, 0, self.len()-1)
    }
    fn _query(&self, query_start:usize, query_end:usize, node_index:usize, node_start:usize, node_end:usize)->Entry{
        if query_end < node_start || node_end < query_start{return Entry::new(i32::MIN as isize, i32::MIN as isize, 0, i32::MIN as isize)}
        if query_start <= node_start && node_end <= query_end {return self.nodes[node_index].clone()}
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        SegmentTree::merge(&self._query(query_start, query_end, left_child, node_start, mid),
            &self._query(query_start, query_end, right_child, mid+1, node_end)
        )
    }
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut i = 0;
    let mut data = vec![0;n];
    while i < n{
        data[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    let mut seg = SegmentTree::new(n);
    seg.build(&data);
    let mut t:u8;
    let mut s:usize;
    let mut e:isize;
    let mut q:usize = it.next().unwrap().parse().unwrap();
    while q!=0{
        t = it.next().unwrap().parse().unwrap();
        s = it.next().unwrap().parse().unwrap();
        e = it.next().unwrap().parse().unwrap();
        if t==0{
            seg.update(s-1, e);
        }else{
            println!("{}", seg.query(s-1, (e-1) as usize).max)
        }
        q-=1;
    }
}