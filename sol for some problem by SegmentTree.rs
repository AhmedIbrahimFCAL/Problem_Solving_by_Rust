// https://www.youtube.com/redirect?event=video_description&redir_token=QUFFLUhqbHR1enU4d0N1azRGbmdyUmllVS1DZy12VDIzQXxBQ3Jtc0tsdTQyUEpMS2Y5UEt3R1AzQWdqemxGUDk4QWZxeW8ydGR4bXA0VUxlSnNGcS1sY1E3OC02RW1ud1JaeVcxQmF6anNWbjI3aGFuY3h1bUlvcFRGSW5UbTNGeHFKTkJKZnpELTlsRTIwbjlKV2pESl9mTQ&q=https%3A%2F%2Fcodeforces.com%2Fedu%2Fcourse%2F2%2Flesson%2F4%2F1%2Fpractice%2Fcontest%2F273169%2Fproblem%2FB&v=EjjtJMw2dfo
use std::io::{Read, stdin};
// children i: 2i+1 (left), 2i+2 (right)
// parent i: (i-1)/2
struct SegmentTree{
    nodes:Vec<isize>,
    size:usize,
}
impl SegmentTree{
    fn new(n:usize) -> Self{
        Self{nodes: vec![0;(n<<1).next_power_of_two()], size:n}
    }
    // node_index: index for tree, storing min at original data from node_start to node_end
    fn build(&mut self, data:&Vec<isize>){
        self._build(data,0, 0, self.len()-1);
    }
    fn len(&self)->usize{
        self.size
    }
    fn _build(&mut self,data:&Vec<isize>, node_index:usize, node_start:usize, node_end:usize){
        if node_start == node_end{
            self.nodes[node_index] = data[node_start];
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._build(data,left_child, node_start, mid);
        self._build(data,right_child, mid+1,node_end);
        self.nodes[node_index] = self.nodes[left_child].min(self.nodes[right_child]);
    }
    fn update(&mut self,position:usize, value:isize){
        self._update(position, value, 0, 0, self.len()-1);
    }
    fn _update(&mut self,position:usize, value:isize, node_index:usize, node_start:usize, node_end:usize){
        if position < node_start || node_end < position {return}
        if node_start == node_end{
            self.nodes[node_index] = value;
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._update(position, value,left_child, node_start, mid);
        self._update(position, value,right_child, mid+1,node_end);
        self.nodes[node_index] = self.nodes[left_child].min(self.nodes[right_child]);
    }
    fn query(&self, query_start:usize, query_end:usize)->isize{
        self._query(query_start, query_end, 0, 0, self.len()-1)
    }
    fn _query(&self, query_start:usize, query_end:usize, node_index:usize, node_start:usize, node_end:usize)->isize{
        if query_end < node_start || node_end < query_start{return isize::MAX}
        if query_start <= node_start && node_end <= query_end {return self.nodes[node_index]}
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._query(query_start, query_end, left_child, node_start, mid).min(
            self._query(query_start, query_end, right_child, mid+1, node_end)
        ) 
    }
}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut q:usize = it.next().unwrap().parse().unwrap();
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
    let mut e:usize;
    while q!=0{
        t = it.next().unwrap().parse().unwrap();
        s = it.next().unwrap().parse().unwrap();
        e = it.next().unwrap().parse().unwrap();
        if t==1{
            seg.update(s, e as isize);
        }else{
            println!("{}", seg.query(s, e-1))
        }
        q-=1;
    }
}
/* Test Case
5 5
5 4 2 3 5
2 0 3
1 2 6
2 0 3
1 3 1
2 0 5
*/
/*its Solution 
2
4
1
*/