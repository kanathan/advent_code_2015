

fn main() {
    
    println!("{}",get_length(3113322113, 40));

    println!("{}",get_length(3113322113, 50));
}


fn get_length(input: usize, steps: usize) -> usize {
    let mut input_str = format!("{}",input);
    
    fn sequence(input: &str) -> String {
        let mut output = String::new();
        let mut input_iter = input.chars().peekable();
        while input_iter.peek().is_some() {
            let c = input_iter.next().unwrap();
            let mut count = 1;
            while input_iter.peek() == Some(&c) {
                input_iter.next();
                count += 1;
            }
            output.push_str(&format!("{}{}",count,c));
        }
        output
    }

    for _ in 0..steps {
        input_str = sequence(&input_str);
    }

    input_str.len()
}



#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test1() {
        assert_eq!(get_length(1, 5), 6);
    }
}