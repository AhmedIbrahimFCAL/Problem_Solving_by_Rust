use std::io::{Read, stdin};

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut grid = vec![vec![0;n];n];
    let mut i =0;
    let mut j ;
    while i<n{
        j=0;
        let txt = it.next().unwrap().as_bytes();
        while j<n{
            grid[i][j] = txt[j];
            j+=1;
        }
        i+=1;
    }
    let mut dp = vec![vec![0;n];n];
    i=n;
    if grid[n-1][n-1] == b'.'{
        dp[n-1][n-1] = 1;
    }
    while i>0{
        j=n;
        i-=1;
        while j>0{
            j-=1;
            if grid[i][j]==b'*' || i==n-1 && j==n-1{
                continue;
            }
            if i==n-1{
                dp[i][j]+=dp[i][j+1];
            }else if j==n-1{
                dp[i][j]+=dp[i+1][j];
            }else{
                dp[i][j]+=dp[i][j+1];
                dp[i][j]+=dp[i+1][j];
                dp[i][j]%=1000000007;
            }
        }
    }
    println!("{}",dp[0][0]);
}

/* recursive solution
use std::io::{Read, stdin};
fn count_paths(grid:&mut Vec<Vec<u8>>,dp:&mut Vec<Vec<i64>>,i:usize,j:usize)->usize{
    if i==grid.len()-1 && j==grid.len()-1 && grid[i][j]==b'.'{return 1;}
    if i==grid.len() || j==grid.len() || grid[i][j]==b'*' {return 0;}
    if dp[i][j]==-1{
        dp[i][j] = ((count_paths(grid, dp,i+1, j) + count_paths(grid,dp, i, j+1))%(1e9+7f64) as usize) as i64;
    }
    return dp[i][j] as usize;
}

fn main(){
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("");
    let mut it = buf.split_whitespace();
    let n:usize = it.next().unwrap().parse().unwrap();
    let mut grid = vec![vec![0;n];n];
    let mut i =0;
    let mut j ;
    while i<n{
        j=0;
        let txt = it.next().unwrap().as_bytes();
        while j<n{
            grid[i][j] = txt[j];
            j+=1;
        }
        i+=1;
    }
    let mut dp = vec![vec![-1;n];n];
    println!("{}",count_paths(&mut grid,&mut dp, 0, 0));
} */