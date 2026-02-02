// https://www.youtube.com/redirect?event=video_description&redir_token=QUFFLUhqbFF2d3lqMklqNEtfam16c3dXbUtzSlVsTW1od3xBQ3Jtc0tuSzZ3bEx3VEtmVmEtc2R3N0t0LTE0cUgyY2pEWVkyaU5fMWNVSjYzNHV1a0JUbG1oRmpWRVY4a0dCN2dHY2xkc1JGelF4MFFtcjhnUjhOV2hDWG5VanY1ZHFUUExNT3J2aURra3BTMVU3b1lmcW9tTQ&q=https%3A%2F%2Fcodeforces.com%2Fedu%2Fcourse%2F2%2Flesson%2F4%2F1%2Fpractice%2Fcontest%2F273169%2Fproblem%2FC&v=EjjtJMw2dfo
use std::io::{Read, stdin};

// children i: 2i+1 (left), 2i+2 (right)
// parent i: (i-1)/2
struct SegmentTree{
    nodes:Vec<(i32, u32)>,
    size:usize,
}
impl SegmentTree{
    fn new(n:usize) -> Self{
        Self{nodes: vec![(0,0);(n<<1).next_power_of_two()], size:n}
    }
    // node_index: index for tree, storing min at original data from node_start to node_end
    fn build(&mut self, data:&Vec<i32>){
        self._build(data,0, 0, self.len()-1);
    }
    fn len(&self)->usize{
        self.size
    }
    fn merge(p1:&(i32,u32),p2:&(i32,u32))->(i32,u32){
        if p1.0 == p2.0{
            (p1.0, p1.1 + p2.1)
        }else{
            *p1.min(p2)
        } 
    }
    fn _build(&mut self,data:&Vec<i32>, node_index:usize, node_start:usize, node_end:usize){
        if node_start == node_end{
            self.nodes[node_index] = (data[node_start], 1);
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._build(data,left_child, node_start, mid);
        self._build(data,right_child, mid+1,node_end);
        self.nodes[node_index] = SegmentTree::merge(&self.nodes[right_child], &self.nodes[left_child])
    }
    fn update(&mut self,position:usize, value:i32){
        self._update(position, value, 0, 0, self.len()-1);
    }
    fn _update(&mut self,position:usize, value:i32, node_index:usize, node_start:usize, node_end:usize){
        if position < node_start || node_end < position {return}
        if node_start == node_end{
            self.nodes[node_index] = (value, 1);
            return;
        }
        let left_child = (node_index<<1)+1;
        let right_child = left_child+1;
        let mid = node_start + ((node_end-node_start)>>1);
        self._update(position, value,left_child, node_start, mid);
        self._update(position, value,right_child, mid+1,node_end);
        self.nodes[node_index] = SegmentTree::merge(&self.nodes[right_child], &self.nodes[left_child])
    }
    fn query(&self, query_start:usize, query_end:usize)->(i32, u32){
        self._query(query_start, query_end, 0, 0, self.len()-1)
    }
    fn _query(&self, query_start:usize, query_end:usize, node_index:usize, node_start:usize, node_end:usize)->(i32, u32){
        if query_end < node_start || node_end < query_start{return (i32::MAX, 0)}
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
            seg.update(s, e as i32);
        }else{
            let ans = seg.query(s, e-1);
            println!("{} {}", ans.0, ans.1)
        }
        q-=1;
    }
}
/* Test Case
5 5
3 4 3 5 2
2 0 3
1 1 2
2 0 3
1 0 2
2 0 5
*/
/* its Solution
3 2
2 1
2 3
*/