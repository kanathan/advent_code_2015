use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = include_str!("input");

    let mut sum = 0;
    for line in input.lines() {
        sum += get_mem_diff(line);
    }
    println!("{}",sum);

    let mut sum = 0;
    for line in input.lines() {
        sum += get_enc_diff(line);
    }
    println!("{}",sum);

}


fn get_mem_diff(line: &str) -> i32 {
    lazy_static! {
        static ref RE_REP: Regex = Regex::new(r#"(?:\\{2})|(?:\\")|(?:\\x[a-fA-F0-9]{2})"#).unwrap();
    };

    let trimmed_line = RE_REP.replace_all(&line[1..line.len()-1], ".").into_owned();

    (line.len() - trimmed_line.len()) as i32
}

fn get_enc_diff(line: &str) -> i32 {
    let encoded_line = line
        .replace(r"\", r"\\")
        .replace(r#"""#, r#"\""#);
    let encoded_line = format!("\"{}\"",encoded_line);

    (encoded_line.len() - line.len()) as i32
}




#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "\"\"\n\
    \"abc\"\n\
    \"aaa\\\"aaa\"\n\
    \"\\x27\"";


    #[test]
    fn test1() {
        let mut sum = 0;
        for line in INPUT.lines() {
            sum += get_mem_diff(line);
        }
        assert_eq!(sum,12);
    }

    #[test]
    fn test2() {
        let mut sum = 0;
        for line in INPUT.lines() {
            sum += get_enc_diff(line);
        }
        assert_eq!(sum,19);
    }
}