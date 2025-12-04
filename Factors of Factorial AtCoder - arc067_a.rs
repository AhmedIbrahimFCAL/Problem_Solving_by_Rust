use std::io::stdin;

fn sieve(nums:&mut Vec<bool>)->usize{
    let f = !nums[2];
    let mut i =2;
    let mut j;
    let mut c = 0;
    while i<nums.len(){
        j=i*i;
        while j<nums.len(){
            nums[j] = f;
            j+=i;
            c+=1;
        }
        i+=1;
    }
    c
}
fn power_of_prime_in_n(mut n:usize,p:usize)->usize{
    let mut count = 0;
    while n!=0{
        n/=p;
        count+=n;
    }
    count
}

fn main(){
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("");
    let n = buf.trim().parse().unwrap();
    if n ==1{
        print!("1");
        return;
    }
    let mut is_prime = vec![true;n+1];
    sieve(&mut is_prime);
    let mut res = 1;
    let mut i=2;
    while i<is_prime.len(){
        if is_prime[i]{
            let p = power_of_prime_in_n(n, i);
            res=(res*(p+1))%1000000007;
        }
        i+=1;
    }
    print!("{res}");
}