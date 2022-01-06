#![allow(non_snake_case)]
use ndarray::Array2;
use regex::Regex;

fn main() {
    let input = include_str!("input");

    println!("Lights: {}",get_light_count(input));
    println!("Lights: {}",get_light_intensity(input));
}

fn get_light_count(input: &str) -> usize {
    let RE = Regex::new(r"([\w\s]+) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let mut grid = Array2::<bool>::default((1000,1000));

    for line in input.lines() {
        let caps = RE.captures(line).expect(&format!("Unable to parse: {}",line));
        let mode = caps.get(1).unwrap().as_str();
        let x1 = caps.get(2).unwrap().as_str().parse().unwrap();
        let y1 = caps.get(3).unwrap().as_str().parse().unwrap();
        let x2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let y2: usize = caps.get(5).unwrap().as_str().parse().unwrap();

        for x in x1..x2+1 {
            for y in y1..y2+1 {
                match mode {
                    "turn on" => *grid.get_mut([x,y]).unwrap() = true,
                    "turn off" => *grid.get_mut([x,y]).unwrap() = false,
                    "toggle" => *grid.get_mut([x,y]).unwrap() ^= true,
                    _ => panic!("Invalid instr: {}",line),
                }
            }
        }
    }

    let mut sum = 0;
    for item in grid.iter() {
        if *item { sum += 1 }
    }
    sum
}

fn get_light_intensity(input: &str) -> usize {
    let RE = Regex::new(r"([\w\s]+) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let mut grid = Array2::<usize>::default((1000,1000));

    for line in input.lines() {
        let caps = RE.captures(line).expect(&format!("Unable to parse: {}",line));
        let mode = caps.get(1).unwrap().as_str();
        let x1 = caps.get(2).unwrap().as_str().parse().unwrap();
        let y1 = caps.get(3).unwrap().as_str().parse().unwrap();
        let x2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let y2: usize = caps.get(5).unwrap().as_str().parse().unwrap();

        for x in x1..x2+1 {
            for y in y1..y2+1 {
                match mode {
                    "turn on" => *grid.get_mut([x,y]).unwrap() += 1,
                    "turn off" => {
                        if *grid.get([x,y]).unwrap() > 0 {
                            *grid.get_mut([x,y]).unwrap() -= 1
                        }
                    }
                    "toggle" => *grid.get_mut([x,y]).unwrap() += 2,
                    _ => panic!("Invalid instr: {}",line),
                }
            }
        }
    }

    grid.sum()
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}