pub fn square_of_sum(n: u32) -> u32 {
    let mut acc = 0;
    
    for i in 1..n+1{
        acc += i;
    }
    
    return acc.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut acc = 0;

    for a in 1..n+1{
        acc += a.pow(2);
    }

    return acc;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
