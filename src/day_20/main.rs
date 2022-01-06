

fn main() {
    let input = 34000000;

    println!("Lowest is {}",find_lowest_house(input));
    println!("Lowest is {}",find_lowest_house_p2(input));
}


fn find_lowest_house(input: usize) -> usize {
    let mut presents: Vec<usize> = vec![0;input/10+1];

    for divisor in 1..=input/10 {
        for house_idx in (divisor..=(input/10)).step_by(divisor) {
            presents[house_idx] += divisor * 10;
        }
    }

    for (idx, val) in presents.iter().enumerate() {
        if *val >= input { return idx }
    }
    0
}


fn find_lowest_house_p2(input: usize) -> usize {
    let mut presents: Vec<usize> = vec![0;input/10+1];

    for divisor in 1..=input/10 {
        let mut house_idx = divisor;
        for _ in 0..50 {
            presents[house_idx] += divisor * 11;
            house_idx += divisor;
            if house_idx >= presents.len() { break }
        }
    }

    for (idx, val) in presents.iter().enumerate() {
        if *val >= input { return idx }
    }
    0
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
    }
}