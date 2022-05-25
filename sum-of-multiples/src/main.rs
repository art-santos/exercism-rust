fn main() {
    println!("{:?}", sum_of_multiples(4, &[3,0]));
}


pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut acc = 0;
    
    for n in 1..limit {
        for factor in factors {
            if factor > &0{
                if n % factor == 0{
                    acc += n;
                    break;
                }
            }
        }
    }

    acc
}