

fn main() {
    let input = include_str!("input.txt");

    let mut floor = 0;
    let mut position = None;

    for (idx,ch) in input.replace("\n", "").chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if position.is_none() && floor == -1 { position = Some(idx+1) }
    }

    println!("Floor {}",floor);
    println!("Position at -1: {:?}",position)
}
