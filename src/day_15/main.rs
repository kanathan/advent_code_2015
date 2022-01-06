use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    println!("{}",get_max_score(input));
    println!("{}",get_max_score_p2(input));
}


fn get_max_score(input: &str) -> i32 {
    let mut max_score = 0;

    let mut ingredients: Vec<HashMap<String, i32>> = Vec::new();
    for line in input.lines() {
        let info = line.split_once(" ").unwrap().1;
        let mut ingr = HashMap::new();
        for item_info in info.split(", ") {
            let (k, v) = item_info.split_once(" ").unwrap();
            ingr.insert(k.to_string(), v.parse::<i32>().unwrap());
        }
        ingredients.push(ingr);
    }

    // Hardcoded for 4 ingredients
    for a in 0..=100 {
        for b in 0..=(100-a) {
            for c in 0..=(100-a-b) {
                let d = 100-a-b-c;

                let mut score = 1;
                for key in ["capacity", "durability", "flavor", "texture"] {
                    let sum = (ingredients[0][key] * a + ingredients[1][key] * b + ingredients[2][key] * c + ingredients[3][key] * d).max(0);
                    score *= sum;
                }
                max_score = max_score.max(score);
            }
        }
    }


    max_score
}

fn get_max_score_p2(input: &str) -> i32 {
    let mut max_score = 0;

    let mut ingredients: Vec<HashMap<String, i32>> = Vec::new();
    for line in input.lines() {
        let info = line.split_once(" ").unwrap().1;
        let mut ingr = HashMap::new();
        for item_info in info.split(", ") {
            let (k, v) = item_info.split_once(" ").unwrap();
            ingr.insert(k.to_string(), v.parse::<i32>().unwrap());
        }
        ingredients.push(ingr);
    }

    // Hardcoded for 4 ingredients
    for a in 0..=100 {
        for b in 0..=(100-a) {
            for c in 0..=(100-a-b) {
                let d = 100-a-b-c;

                let key = "calories";
                let cal_sum = (ingredients[0][key] * a + ingredients[1][key] * b + ingredients[2][key] * c + ingredients[3][key] * d).max(0);
                if cal_sum != 500 { continue }

                let mut score = 1;
                for key in ["capacity", "durability", "flavor", "texture"] {
                    let sum = (ingredients[0][key] * a + ingredients[1][key] * b + ingredients[2][key] * c + ingredients[3][key] * d).max(0);
                    score *= sum;
                }
                max_score = max_score.max(score);
            }
        }
    }


    max_score
}

#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}