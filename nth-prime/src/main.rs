use std::time::Instant;


fn main() {
    let now = Instant::now();
    println!("{}", nth(100));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut acc = 0;
    let mut i: u32 = 2;
    let mut primes_found = 0;

    loop{
        if is_prime(i) {
            acc = i;
            primes_found += 1;
            i += 1;
        }
        i += 1;
        if primes_found == n {
            break;
        }
    }
    return acc
}
