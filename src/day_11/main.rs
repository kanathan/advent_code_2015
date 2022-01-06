use fancy_regex::{Regex, Captures};
use lazy_static::lazy_static;
use itertools::izip;

fn main() {
    let mut password = "vzbxxyzz".to_string();
    password = inc_password(&password);
    while !check_password(&password) {
        password = inc_password(&password);
    }
    println!("{}",password);

}

fn check_password(password: &str) -> bool {
    lazy_static! {
        static ref RE_BADCHAR: Regex = Regex::new(r"[iol]").unwrap();
        static ref RE_PAIR: Regex = Regex::new(r"([a-z])\1").unwrap();
    }
    if RE_BADCHAR.is_match(password).unwrap() { return false }
    if RE_PAIR.find_iter(password).count() < 2 { return false }
    if !izip!(password.chars(), password[1..].chars(), password[2..].chars())
        .any(|(a, b, c)| (a as u32) + 1 == (b as u32) && (b as u32) + 1 == (c as u32)) { return false }

    true
}

fn inc_password(password: &str) -> String {
    lazy_static! {
        static ref RE_BADCHAR: Regex = Regex::new(r"[iol]").unwrap();
        static ref RE: Regex = Regex::new(r"([a-y])(z*)$").unwrap();
    }

    if let Some(match_str) = RE_BADCHAR.find(password).unwrap() {
        let pos = password.find(match_str.as_str()).unwrap();
        let password: String = password.chars()
            .enumerate()
            .map(|(idx, c)| {
                if idx < pos { c }
                else if idx == pos { char::from_u32((c as u32) + 1).unwrap() }
                else { 'a' }
            })
            .collect();
        return password;
    }

    RE.replace(password, |caps: &Captures| {
        let chr_1 = std::char::from_u32(caps[1].chars().next().unwrap() as u32 + 1).unwrap();
        let str_2 = {
            let mut z_string = String::new();
            for _ in 0..caps[2].len() {
                z_string.push('a');
            }
            z_string
        };
        format!("{}{}",chr_1,str_2)
    }).into_owned()
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test1() {
        let password = "ghijklmn";
        println!("{}",inc_password(password));
        //println!("{}",check_password(password));
    }
}