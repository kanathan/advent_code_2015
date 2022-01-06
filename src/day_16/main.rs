use std::collections::HashMap;



fn main() {
    let input = include_str!("input");

    const TRUTH_INPUT: &str = 
    "children: 3, \
    cats: 7, \
    samoyeds: 2, \
    pomeranians: 3, \
    akitas: 0, \
    vizslas: 0, \
    goldfish: 5, \
    trees: 3, \
    cars: 2, \
    perfumes: 1";

    let truth_dict = parse_dict(TRUTH_INPUT);
    let sue_dicts = parse_input(input);

    for (num, test_dict) in sue_dicts.iter() {
        if test_dict.iter().all(|(k, v)| truth_dict.contains_key(k) && truth_dict[k] == *v) {
            println!("Sue is {}", num);
            break;
        }
    }

    for (num, test_dict) in sue_dicts.iter() {
        if test_dict
            .iter()
            .all(|(k, v)|
            {
                truth_dict.contains_key(k) &&
                match k.as_str() {
                    "cats" | "trees" => *v > truth_dict[k],
                    "pomeranians" | "goldfish" => *v < truth_dict[k],
                    _ =>  *v == truth_dict[k],
                }
                
            }) {
            println!("Sue is {}", num);
            break;
        }
    }

}

fn parse_input(input: &str) -> HashMap<usize, HashMap<String, u8>> {
    let mut output = HashMap::new();
    for line in input.lines() {
        let (sue_str, dict_str) = line.split_once(": ").unwrap();
        let sue_num = sue_str.split_once(" ").unwrap().1.parse().unwrap();
        output.insert(sue_num, parse_dict(dict_str));
    }
    output
}

fn parse_dict(input: &str) -> HashMap<String, u8> {
    let mut dict = HashMap::new();

    for (k, v) in input.split(", ").map(|s| s.split_once(": ").unwrap()) {
        dict.insert(k.to_string(), v.parse::<u8>().unwrap());
    }

    dict
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}