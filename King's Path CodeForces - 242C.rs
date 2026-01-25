use std::{collections::{HashMap, VecDeque}, io::{Read, stdin}};
/*
 2345678901
5-----#-...
6.....----#
*/
fn bfs(grid:&mut HashMap<(u32,u32),u32>, x0:u32,y0:u32, x1:u32,y1:u32){
    *grid.get_mut(&(x0,y0)).unwrap() = 0;
    let mut queue = VecDeque::new();
    queue.push_back((x0,y0));
    while !queue.is_empty(){
        let u = queue.pop_front().unwrap();
        let dist = *grid.get(&u).unwrap() + 1;
        for i in [-1,1,0]{
            for j in [-1,1,0]{
                let x = u.0.wrapping_add(i as u32);
                let y = u.1.wrapping_add(j as u32);
                if let Some(v) = grid.get_mut(&(x,y)) && *v == u32::MAX{
                    *v = dist;
                    if (x,y) == (x1,y1){
                        return;
                    }
                    queue.push_back((x,y));
                }
            }
        }
    }

}
fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let x0 = it.next().unwrap().parse::<u32>().unwrap();
    let y0 = it.next().unwrap().parse::<u32>().unwrap();
    let x1 = it.next().unwrap().parse::<u32>().unwrap();
    let y1 = it.next().unwrap().parse::<u32>().unwrap();
    let n = it.next().unwrap().parse::<u32>().unwrap();
    let mut grid = HashMap::new();
    let mut i = 0;
    while i < n{
        let r = it.next().unwrap().parse::<u32>().unwrap();
        let mut a = it.next().unwrap().parse::<u32>().unwrap();
        let b = it.next().unwrap().parse::<u32>().unwrap();
        while a <= b{
            grid.insert((r,a),u32::MAX);
            a+=1;
        }
        i+=1;
    }
    if grid.contains_key(&(x0,y0)) && grid.contains_key(&(x1,y1)){
        bfs(&mut grid, x0, y0, x1, y1);
        if let Some(d) = grid.get(&(x1,y1)){
            if *d != u32::MAX{
                println!("{d}");
                return;
            }
        }
    }
    println!("-1");
}