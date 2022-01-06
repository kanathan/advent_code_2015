use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let packages: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    println!("{}",solve(&packages, packages.iter().sum::<usize>()/3));
    println!("{}",solve(&packages, packages.iter().sum::<usize>()/4));
}


fn solve(packages: &Vec<usize>, target: usize) -> usize {
    let mut output = None;

    for group_1_size in 1..packages.len() {
        for permutation in packages.iter().combinations(group_1_size) {
            if permutation.iter().map(|x| **x).sum::<usize>() != target { continue }

            let group_1 = &permutation[..group_1_size];
            let qe = group_1.iter().map(|v| **v).product::<usize>();
            if let Some(old_qe) = output {
                if old_qe > qe { output = Some(qe) }
            } else {
                output = Some(qe)
            }
        }

        if output.is_some() { break }
    }

    output.unwrap()
}



/*fn get_qe(packages: &Vec<u32>) -> u32 {
    let group_sum = packages.iter().sum::<u32>() / 3;

    let mut valid_groups: Vec<HashBag<u32>> = Vec::new();

    for group_size in 1..packages.len() {
        for group_1 in packages.into_iter().permutations(group_size) {
            if group_1.iter().map(|x| **x).sum::<u32>() != group_sum { continue }
            valid_groups.push(group_1.into_iter().map(|x| *x).collect());

            /*let mut groups_2_3 = packages.clone();
            for val in group_1.iter() {
                groups_2_3.remove(val);
            }
            'group_2: for group_2_size in 1..groups_2_3.len() {
                for group_2 in groups_2_3.iter().permutations(group_2_size) {
                    if group_2.into_iter().sum::<u32>() == group_sum {
                        valid_groups.push(group_1.into_iter().map(|x| *x).collect());
                        break 'group_2;
                    }
                }
            }*/
        }
        if !valid_groups.is_empty() { break }
        println!("No matches for group 1 size {}",group_size);
    }

    valid_groups.iter()
        .map(|bag| bag.iter().product::<u32>())
        .min().unwrap()
}*/


#[cfg(test)]
mod test {
    //use crate::*;



    #[test]
    fn test1() {
    }
}