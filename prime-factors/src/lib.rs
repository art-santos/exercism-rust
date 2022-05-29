pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut j: u64 = n;
    let mut i: u64 = 2;
    
    loop{
        if j % i == 0 {
            factors.push(i);
            j = j / i;
        } else {
            i += 1;
        }
        if j == 1 {
            break;
        }
    }
    factors
}

