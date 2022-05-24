pub fn square(s: u32) -> u64 {
    if s >= 1 && s < 65 {
       if s == 1 {
           1
       } else {
           2u64.pow(s - 1)
       }
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    let mut acc = 0;
    for n in 1..65 {
        acc += square(n);
    }
    acc
}
