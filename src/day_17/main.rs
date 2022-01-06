use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let containers = input.lines().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    /*let count = containers
        .iter()
        .permutations(containers.len())
        .filter(|seq| {
            let mut sum = 0;
            for val in seq.iter() {
                sum += **val;
                if sum == 150 { return true }
                if sum > 150 { return false }
            }
            false
        })
        .count();*/
    let mut count = 0;
    let mut min_containers = containers.len();
    for i in 0..containers.len() {
        for combination in containers.iter().combinations(i) {
            if combination.iter().map(|v| **v).sum::<usize>() == 150 {
                min_containers = min_containers.min(i);
                count += 1;
            }
        }
    }
    let mut min_count = 0;
    for combination in containers.iter().combinations(min_containers) {
        if combination.iter().map(|v| **v).sum::<usize>() == 150 {
            min_count += 1;
        }
    }

    println!("{} combinations",count);
    println!("{} min containers with {} combinations",min_containers, min_count);



}




#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}