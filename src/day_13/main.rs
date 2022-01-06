use std::collections::HashMap;
use regex::Regex;
use ndarray::Array2;
use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let table = parse_input(input);
    println!("{}",get_best(&table));

    println!("{:?}",table);
    let mut new_table = Array2::zeros((table.shape()[0]+1,table.shape()[0]+1));
    for x in 0..table.shape()[0] {
        for y in 0..table.shape()[1] {
            new_table[[x,y]] = table[[x,y]];
        }
    }
    println!("{:?}",new_table);
    println!("{}",get_best(&new_table));
}


fn get_best(table: &Array2<i32>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();
    for order in (0..table.shape()[0]).permutations(table.shape()[0]) {
        let mut score = 0;
        for (idx, pos) in order.iter().enumerate() {
            let n1_idx = if idx > 0 { order[idx - 1] } else { order[order.len()-1] };
            let n2_idx = if idx < order.len()-1 { order[idx + 1] } else { order[0] };
            score += table[[*pos,n1_idx]] + table[[*pos,n2_idx]];
        }
        scores.push(score);
    }

    *scores.iter().max().unwrap()
}


fn parse_input(input: &str) -> Array2<i32> {
    let re = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();

    let mut people_idx_map = HashMap::new();

    for line in input.lines() {
        let (a,_b) = line.split_once(" ").unwrap();
        if !people_idx_map.contains_key(a) { people_idx_map.insert(a.to_string(), people_idx_map.len()); }
    }

    let mut table = Array2::zeros((people_idx_map.len(),people_idx_map.len()));
    

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let pri_name = caps.get(1).unwrap().as_str();
        let gainloss = caps.get(2).unwrap().as_str();
        let val_str = caps.get(3).unwrap().as_str();
        let sec_name = caps.get(4).unwrap().as_str();

        let pri_idx = people_idx_map[pri_name];
        let sec_idx = people_idx_map[sec_name];
        let happyness: i32 = if gainloss == "gain" { val_str.parse().unwrap() } else { val_str.parse::<i32>().unwrap() * -1 };

        table[[pri_idx,sec_idx]] = happyness;
    }

    table
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "Alice would gain 54 happiness units by sitting next to Bob.\n\
    Alice would lose 79 happiness units by sitting next to Carol.\n\
    Alice would lose 2 happiness units by sitting next to David.\n\
    Bob would gain 83 happiness units by sitting next to Alice.\n\
    Bob would lose 7 happiness units by sitting next to Carol.\n\
    Bob would lose 63 happiness units by sitting next to David.\n\
    Carol would lose 62 happiness units by sitting next to Alice.\n\
    Carol would gain 60 happiness units by sitting next to Bob.\n\
    Carol would gain 55 happiness units by sitting next to David.\n\
    David would gain 46 happiness units by sitting next to Alice.\n\
    David would lose 7 happiness units by sitting next to Bob.\n\
    David would gain 41 happiness units by sitting next to Carol.";


    #[test]
    fn test1() {
        let table = parse_input(INPUT);
        println!("{}",get_best(&table));
    }
}