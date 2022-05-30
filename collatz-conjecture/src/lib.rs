pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    } else { 
        let mut mut_n: u64 = n as u64;
        let mut count: u64 = 0;        
            while mut_n > 1 {
                if mut_n % 2 == 0 {
                    mut_n = mut_n / 2;
                } else {
                    if mut_n >= u64::MAX/3 {
                        return None;
                    } else {
                    mut_n = mut_n * 3 + 1;
                    }
                }
                count += 1;
            }
            return Some(count);
    }
}

