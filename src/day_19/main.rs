use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");

    println!("{}",get_unique_count(input));
    println!("{}",get_reduce_count(input));
}


fn get_unique_count(input: &str) -> usize {
    let mut formulas = HashSet::new();
    let (template, rules) = parse_input_count(input);

    for (key, replacements) in rules.iter() {
        let mut start = 0;

        while let Some(idx) = template[start..].find(key) {
            let (l,r) = template.split_at(start+idx);
            let r = {
                let mut c = r.chars();
                for _ in 0..key.len() {
                    c.next();
                }
                c.as_str()
            };
            for repl in replacements.iter() {
                let new_string = format!("{}{}{}",l,repl,r);
                formulas.insert(new_string);
            }
            start += 1;
        }
    }

    formulas.len()
}

fn get_reduce_count(input: &str) -> usize {
    // From https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/

    let (template, rules) = parse_input_count(input);

    let mut element_count = 0;
    for element in rules.keys() {
        element_count += template.matches(element).count()
    }
    //let rn_ar_count = template.matches("Rn").count() + template.matches("Ar").count();
    let y_count = template.matches("Y").count();

    element_count - y_count - 1
}

/*fn get_min_max_opt(input: &str, steps: usize) -> usize {
    let (template, rules) = parse_input_count(input);
    let mut pair_count = HashMap::new();

    for idx in 0..template.len()-1 {
        *pair_count.entry(template[idx..idx+2].to_string()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_pair_count = HashMap::new();
        for (key, val) in pair_count.iter() {
            let pair = format!("{}{}",key.chars().nth(0).unwrap(),rules[key]);
            *new_pair_count.entry(pair).or_insert(0) += val;
            let pair = format!("{}{}",rules[key],key.chars().nth(1).unwrap());
            *new_pair_count.entry(pair).or_insert(0) += val;
        }
        pair_count = new_pair_count;
    }

    let mut char_counts = HashMap::new();
    for (key, val) in pair_count.iter() {
        *char_counts.entry(key.chars().nth(0).unwrap()).or_insert(0) += val;
    }
    *char_counts.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    let mut count_vec: Vec<_> = char_counts.iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(b.1));

    *count_vec.iter().last().unwrap().1 - *count_vec.iter().next().unwrap().1
}*/


fn parse_input_count(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let mut lines = input.lines();
    let mut rules = HashMap::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() { break }
        let (k, v) = line.split_once(" => ").expect(&format!("Invalid input: {}",line));
        rules.entry(k.to_string()).or_insert(Vec::new()).push(v.to_string());
    }

    let template = lines.collect();

    (template, rules)
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "H => HO\n\
    H => OH\n\
    O => HH\n\
    \n\
    HOH";

    const INPUT2: &str =
    "H => HO\n\
    H => OH\n\
    O => HH\n\
    \n\
    HOHOHO";

    #[test]
    fn test1() {
        assert_eq!(get_unique_count(INPUT), 4);
        assert_eq!(get_unique_count(INPUT2), 7);
    }

    #[test]
    fn test2() {
        //assert_eq!(get_min_max_opt(INPUT, 40), 2188189693529);
    }
}