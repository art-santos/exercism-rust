pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = vec![];  
    if list.len() > 0 {
        for n in 0..list.len()-1 {
            proverb.push(format!("For want of a {} the {} was lost.", list[n], list[n+1]));
            println!("{}", n);
        }
        proverb.push(format!("And all for the want of a {}.", list[0]));

        return proverb.join("\n")
    }else{
        return String::new();
    }    
}