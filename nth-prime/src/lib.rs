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
    let mut acc = 2;
    let mut i: u32 = 2;
    let mut primes_found = 0;

    while primes_found <= n {
        if is_prime(i) {
            acc = i;
            primes_found += 1;
        }
        i += 1;
    }

    return acc
}