fn trim(string: &str) -> Vec<char> {
    let chars = string.chars().collect::<Vec<char>>();
    let mut new_chars: Vec<char> = vec![];
    for c in chars {
        match c {
            '(' => new_chars.push(c),
            ')' => new_chars.push(c),
            '[' => new_chars.push(c),
            ']' => new_chars.push(c),
            '{' => new_chars.push(c),
            '}' => new_chars.push(c),
            _ => {}
        }
    }
    return new_chars;
}

pub fn brackets_are_balanced(string: &str) -> bool{
    //convert string to vector of chars
    let chars = trim(string);
    let mut brackets_left = vec![];
    let mut brackets_right = vec![];
    let mut parentheses_left = vec![];
    let mut parentheses_right = vec![];
    let mut braces_left = vec![];
    let mut braces_right = vec![];

    let mut balanced = true;
    
    for n in 0..chars.len(){
        match chars[n] {
            '(' => parentheses_left.push(n),
            ')' => parentheses_right.push(n),
            '[' => braces_left.push(n),
            ']' => braces_right.push(n),
            '{' => brackets_left.push(n),
            '}' => brackets_right.push(n),
            _ => {}
        }
    }

    if brackets_left.len() == brackets_right.len() {
        for n in 0..brackets_left.len(){
            if brackets_left[n] > brackets_right[n] {  
                println!("{} <- HERE", chars[brackets_left[n]]);          
                    balanced = false;
            }
            if brackets_left[n] < chars.len()/2 && brackets_right[n] > chars.len()/2 {
                println!("{} <- HERE1", chars[brackets_left[n]]);   
                if brackets_right[brackets_right.len()-1]-n != chars.len()-1-n{
                    println!("{} <- HERE2", chars[brackets_left[n]]);   
                    balanced = false;
                }
            }
        }
    } else {
        balanced = false;
    }

    if braces_left.len() == braces_right.len() {
        for n in 0..braces_left.len(){
            if braces_left[n] > braces_right[n] {   
                println!("{} <- HERE1", chars[brackets_left[n]]);            
                    balanced = false;
            }
        }
    } else {
        balanced = false;
    }


    if parentheses_left.len() == parentheses_right.len() {
        for n in 0..parentheses_left.len(){
            if parentheses_left[n] > parentheses_right[n] {
                    balanced = false;
            }
            if parentheses_left[n] < chars.len()/2 && parentheses_right[n] > chars.len()/2 {
                if parentheses_right[parentheses_right.len()-1]-n != chars.len()-1-n{
                    balanced = false;
                }
            }
        }
    } else {
        balanced = false;
    }

    balanced
}
