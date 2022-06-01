use rand::prelude::*;

fn main() {
    println!("{}", private_key(10));
    
}


pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(2..p);
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    return g.pow(a as u32) % p;
}