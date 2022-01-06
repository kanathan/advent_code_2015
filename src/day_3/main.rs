use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    println!("Houses visited: {}",travel(input));
    println!("Houses visited with robot: {}",robo_travel(input));
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn travel(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut coords = Coordinates { x: 0, y: 0 };
    houses.insert(coords);

    for ch in input.replace("\n", "").chars() {
        match ch {
            '>' => coords.x += 1,
            '<' => coords.x -= 1,
            '^' => coords.y += 1,
            'v' => coords.y -= 1,
            _ => (),
        }
        houses.insert(coords);
    }

    houses.len()
}

fn robo_travel(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut coords = Coordinates { x: 0, y: 0 };
    let mut robo_coords = Coordinates { x: 0, y: 0 };
    houses.insert(coords);
    let mut robo_turn = false;

    for ch in input.replace("\n", "").chars() {
        let coords_ref = if robo_turn {
            &mut robo_coords
        } else {
            &mut coords
        };
        robo_turn = !robo_turn;
        match ch {
            '>' => coords_ref.x += 1,
            '<' => coords_ref.x -= 1,
            '^' => coords_ref.y += 1,
            'v' => coords_ref.y -= 1,
            _ => (),
        }
        houses.insert(*coords_ref);
    }

    houses.len()
}

#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}