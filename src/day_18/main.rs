
use ndarray::Array2;

fn main() {
    let input = include_str!("input");

    let count = run(input, 100)
        .iter()
        .map(|v| if *v { 1 } else { 0 })
        .sum::<usize>();
    
    println!("{}",count);

    let count = run_stuck(input, 100)
        .iter()
        .map(|v| if *v { 1 } else { 0 })
        .sum::<usize>();
    
    println!("{}",count);

}


fn run(input: &str, steps: usize) -> Array2<bool> {
    let mut grid = parse_input(input);

    for _ in 0..steps {
        step(&mut grid);
    }

    grid
}


fn run_stuck(input: &str, steps: usize) -> Array2<bool> {
    let mut grid = parse_input(input);

    for _ in 0..steps {
        step_stuck(&mut grid);
    }

    grid
}


fn parse_input(input: &str) -> Array2<bool> {
    let shape = (input.lines().next().unwrap().len(), input.lines().count());
    let mut grid = Array2::default(shape);

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => grid[[x,y]] = true,
                '.' => grid[[x,y]] = false,
                _ => panic!("Invalid input")
            }
        }
    }

    grid
}

fn step(grid: &mut Array2<bool>) {
    let old_grid = grid.clone();

    for y in 0..old_grid.shape()[1] {
        for x in 0..old_grid.shape()[0] {
            let count = get_neighbor_count(&old_grid, x, y);
            grid[[x,y]] = 
                if old_grid[[x,y]] {
                    count == 2 || count == 3
                }
                else {
                    count == 3
                };
        }
    }
}

fn step_stuck(grid: &mut Array2<bool>) {
    let xsize = grid.shape()[0];
    let ysize = grid.shape()[1];
    grid[[0,0]] = true;
    grid[[0,ysize-1]] = true;
    grid[[xsize-1,0]] = true;
    grid[[xsize-1,ysize-1]] = true;

    let old_grid = grid.clone();

    for y in 0..old_grid.shape()[1] {
        for x in 0..old_grid.shape()[0] {
            let count = get_neighbor_count(&old_grid, x, y);
            grid[[x,y]] = 
                if old_grid[[x,y]] {
                    count == 2 || count == 3
                }
                else {
                    count == 3
                };
        }
    }

    grid[[0,0]] = true;
    grid[[0,ysize-1]] = true;
    grid[[xsize-1,0]] = true;
    grid[[xsize-1,ysize-1]] = true;
}

fn get_neighbor_count(grid: &Array2<bool>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for ny in y.checked_sub(1).unwrap_or(0)..=(y+1) {
        for nx in x.checked_sub(1).unwrap_or(0)..=(x+1) {
            if nx==x && ny==y { continue }
            if let Some(val) = grid.get((nx,ny)) { if *val { count += 1 }}
        }
    }
    count
}

/*fn print_grid(grid: &Array2<bool>) {
    for y in 0..grid.shape()[1] {
        for x in 0..grid.shape()[0] {
            if grid[[x,y]] { print!("{}",'#') } else { print!("{}",'.') }
        }
        println!();
    }
}*/


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    ".#.#.#\n\
    ...##.\n\
    #....#\n\
    ..#...\n\
    #.#..#\n\
    ####..";


    #[test]
    fn test1() {
        let count = run(INPUT, 4)
            .iter()
            .map(|v| if *v { 1 } else { 0 })
            .sum::<usize>();
        
        assert_eq!(count, 4);
    }
}