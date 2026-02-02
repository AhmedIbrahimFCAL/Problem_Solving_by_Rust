use std::io::{Read, stdin};

struct MonoStack<T>{
    data:Vec<(T, T)>,
}
impl<T:Ord + Copy> MonoStack<T>{
    fn new()->Self{
        Self {
            data: Vec::<(T, T)>::new()
        }
    }
    fn push(&mut self, item: T){
        self.data.push((item,
            if self.data.is_empty() {item} else {item.max(self.data.last().unwrap().1)}
        ));
    }
    fn pop(&mut self)->Option<T>{
        if self.data.is_empty(){
            None
        }else {
            Some(self.data.pop().unwrap().0)
        }
    }
    fn is_empty(&self)->bool{
        self.data.is_empty()
    }
    fn len(&self) -> usize{
        self.data.len()
    }
    fn top(&self) -> Option<T>{
        if self.is_empty() {
            None
        }else{
            Some(self.data.last().unwrap().0)
        }
    }
    fn get_max(&self) -> Option<T>{
        if self.data.is_empty(){
            None
        }else{
            Some(self.data.last().unwrap().1)
        }
    }
}

struct MonoQueue<T>{
    mono_stack_in:MonoStack<T>,
    mono_stack_out:MonoStack<T>,
}
impl<T:Ord + Copy> MonoQueue<T>{
    fn new()->Self{
        Self { mono_stack_in: MonoStack::<T>::new(), mono_stack_out: MonoStack::<T>::new() }
    }
    fn push(&mut self, item:T){
        self.mono_stack_in.push(item);
    }
    fn is_empty(&self) -> bool{
        self.mono_stack_in.is_empty() && self.mono_stack_out.is_empty()
    }
    fn len(&self)->usize{
        self.mono_stack_in.len() + self.mono_stack_out.len()
    }
    fn move_in_to_out(&mut self){
        while !self.mono_stack_in.is_empty(){
            self.mono_stack_out.push(
                self.mono_stack_in.pop().unwrap()
            );
        }
    }
    fn top(&mut self)->Option<T>{
        if self.is_empty(){
            None
        }else{
            if self.mono_stack_out.is_empty(){
                self.move_in_to_out();
            }
            self.mono_stack_out.top()
        }
    }
    fn pop(&mut self)->Option<T>{
        if self.is_empty(){
            None
        }else{
            if self.mono_stack_out.is_empty() {self.move_in_to_out()};
            self.mono_stack_out.pop()
        }
    }
    fn get_max(&self)->Option<T>{
        if self.is_empty(){
            None
        }else{ 
            Some(
                if self.mono_stack_in.is_empty(){   
                    self.mono_stack_out.get_max().unwrap()
                }else if self.mono_stack_out.is_empty(){
                    self.mono_stack_in.get_max().unwrap()
                }else{
                    self.mono_stack_in.get_max().unwrap().max(
                        self.mono_stack_out.get_max().unwrap()
                    )
                }
            )
        }
    }
}
/* another implementation for Monotonic Queue
use std::collections::VecDeque;
struct MonoQueue<T>{
    data:VecDeque<T>,
    maxes:VecDeque<T>
}
impl<T: Ord + Copy> MonoQueue<T>{
    fn new()->Self{
        Self { data: VecDeque::new(), maxes: VecDeque::new() }
    }

    fn push(&mut self, value: T){
        self.data.push_back(value);
        while !self.maxes.is_empty() && *self.maxes.back().unwrap() < value{
            self.maxes.pop_back();
        }
        self.maxes.push_back(value);
    }
    fn pop(&mut self)->Option<T>{
        if self.data.is_empty(){
            None
        }else{
            if *self.data.front().unwrap() == *self.maxes.front().unwrap(){
                self.maxes.pop_front();
            }
            self.data.pop_front()
        }
    }
    fn top(&self)->Option<T>{
        if self.data.is_empty(){
            None
        }else{
            Some(*self.data.front().unwrap())
        }
    }
    fn len(&self)->usize{
        self.data.len()
    }
    fn is_empty(&self)->bool{
        self.data.is_empty()
    }
    fn get_max(&self)->Option<T>{
        if self.maxes.is_empty(){
            None
        }else{
            Some(*self.maxes.front().unwrap())
        }
    }
}
*/
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut q = it.next().unwrap().parse::<usize>().unwrap();
    let mut arr = vec![0u32;n];
    let mut i = 0;
    while i < n{
        arr[i] = it.next().unwrap().parse().unwrap();
        i+=1;
    }
    let mut d;
    while q!=0{
        d = it.next().unwrap().parse().unwrap();
        let mut mq = MonoQueue::new();
        i = 0;
        while i < d{
            mq.push(arr[i]);
            i+=1;
        }
        let mut mn = mq.get_max().unwrap();
        while i < n{
            mq.push(arr[i]);
            mq.pop();
            mn = mn.min(mq.get_max().unwrap());
            i+=1;
        }
        println!("{mn}");
        q-=1;
    }
}