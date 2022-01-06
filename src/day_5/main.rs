#![allow(non_snake_case)]

use fancy_regex::Regex;


fn main() {
    let input = include_str!("input");

    
    let RE_VOWELS = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let RE_TWICE = Regex::new(r"(\w)\1").unwrap();
    let RE_MISSING = Regex::new(r"(?:ab)|(?:cd)|(?:pq)|(?:xy)").unwrap();

    let mut nice_count = 0;
    for line in input.lines() {
        if RE_VOWELS.is_match(line).unwrap() && RE_TWICE.is_match(line).unwrap() && !RE_MISSING.is_match(line).unwrap() {
            nice_count += 1;
        }
    }

    println!("Nice strings: {}",nice_count);


    let RE_TWICE_V2 = Regex::new(r"(\w{2}).*\1").unwrap();
    let RE_REPEAT = Regex::new(r"(\w)\w\1").unwrap();

    let mut nice_count = 0;
    for line in input.lines() {
        if RE_TWICE_V2.is_match(line).unwrap() && RE_REPEAT.is_match(line).unwrap() {
            nice_count += 1;
        }
    }

    println!("Nice strings v2: {}",nice_count);
}




#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}