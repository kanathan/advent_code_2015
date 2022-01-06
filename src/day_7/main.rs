#![allow(non_snake_case)]
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("input");

    let nodes = calc_circuit(input, None);
    println!("{}",nodes["a"]);

    let override_nodes = HashMap::from([
        ("b".to_string(), nodes["a"])
    ]);
    let nodes = calc_circuit(input, Some(override_nodes));
    println!("{}",nodes["a"]);
}


fn calc_circuit(input: &str, overrides: Option<HashMap<String,u16>>) -> HashMap<String,u16> {
    let RE_VAL = Regex::new(r"^\d+$").unwrap();
    let RE_VAR = Regex::new(r"^\w+$").unwrap();
    let RE_OP = Regex::new(r"^([\w\d]*?)\s*(\w+) ([\w\d]+)$").unwrap();


    let mut node_vals: HashMap<String, u16> = HashMap::new();
    let mut node_instrs: HashMap<String,(String, String, String)> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").expect("Unable to parse");

        if overrides.is_some() && overrides.as_ref().unwrap().contains_key(right) {
            node_vals.insert(right.to_string(), overrides.as_ref().unwrap()[right]);
            continue;
        }

        if RE_VAL.is_match(left) {
            node_vals.insert(right.to_string(), left.parse().expect(&format!("Unable to parse: {}",line)));
        } else if RE_VAR.is_match(left) {
            node_instrs.insert(right.to_string(), ("SET".to_string(), String::new(), left.to_string()));
        } else {
            let caps = RE_OP.captures(left).expect(&format!("Unable to capture: {}",line));
            let op = caps.get(2).unwrap().as_str().to_string();
            let a = caps.get(1).unwrap().as_str().to_string();
            let b = caps.get(3).unwrap().as_str().to_string();
            node_instrs.insert(right.to_string(), (op, a, b));
        }
    }

    while !node_instrs.is_empty() {
        let original_size = node_instrs.len();
        node_instrs.retain(|key, (op, a, b)| {
            let a = 
                if a.is_empty() { 0 }
                else if RE_VAL.is_match(a) { a.parse().unwrap() }
                else {
                    if let Some(val) = node_vals.get(a) { *val } else { return true }
                };
            let b = 
                if b.is_empty() { 0 }
                else if RE_VAL.is_match(b) { b.parse().unwrap() }
                else {
                    if let Some(val) = node_vals.get(b) { *val } else { return true }
                };

            match op.as_str() {
                "NOT" => { node_vals.insert(key.clone(), !b); },
                "SET" => { node_vals.insert(key.clone(), b); },
                "AND" => { node_vals.insert(key.clone(), a&b);},
                "OR" => { node_vals.insert(key.clone(), a|b);},
                "LSHIFT" => { node_vals.insert(key.clone(), a << b);},
                "RSHIFT" => { node_vals.insert(key.clone(), a >> b);},
                _ => panic!("Unknown op: {}",op),
            }
            false
        });
        if node_instrs.len() == original_size {
            println!("May be stuck in loop!");
            println!("{:?}",node_vals);
            println!("{:?}",node_instrs);
            break;
        }
    }

    node_vals
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "123 -> x\n\
    456 -> y\n\
    x AND y -> d\n\
    x OR y -> e\n\
    x LSHIFT 2 -> f\n\
    y RSHIFT 2 -> g\n\
    NOT x -> h\n\
    NOT y -> i";


    #[test]
    fn test1() {
        assert_eq!(calc_circuit(INPUT, None)["h"], 65412);
    }
}