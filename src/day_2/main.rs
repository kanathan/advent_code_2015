

fn main() {
    let input = include_str!("input.txt");

    println!("{}",get_paper_amount(input));
    println!("{}",get_ribbon_amount(input));
}

fn get_paper_amount(input: &str) -> usize {
    let mut sqr_ft = 0;
    for line in input.lines() {
        let mut dimensions: Vec<usize> = line.splitn(3, 'x').map(|s| s.parse::<usize>().unwrap()).collect();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        let mut areas = vec![
            2*l*w,
            2*w*h,
            2*h*l,
        ];
        areas.sort();
        dimensions.sort();
        sqr_ft += areas[0] + areas[1] + areas[2] + dimensions[0]*dimensions[1];
    }
    sqr_ft
}

fn get_ribbon_amount(input: &str) -> usize {
    let mut length = 0;
    for line in input.lines() {
        let mut dimensions: Vec<usize> = line.splitn(3, 'x').map(|s| s.parse::<usize>().unwrap()).collect();
        dimensions.sort();

        length += 2 * dimensions[0] + 2 * dimensions[1] + dimensions[0]*dimensions[1]*dimensions[2];
    }
    length
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {

    }
}