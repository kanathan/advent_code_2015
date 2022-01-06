use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input");

    let mut lines = input.lines();

    let boss = Entity {
        hp: lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap(),
        damage: lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap(),
        armor: 0,
    };
    
    println!("{}",get_lowest_mana(&boss, false));
    println!("{}",get_lowest_mana(&boss, true));
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    world: World,
    mana_used: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mana_used.cmp(&self.mana_used)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct World {
    player: Entity,
    boss: Entity,
    timers: Timers,
    cur_mana: u32,
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Entity {
    hp: i32,
    damage: i32,
    armor: i32
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Timers {
    shield: i32,
    poison: i32,
    recharge: i32
}


fn get_lowest_mana(boss: &Entity, hard_mode: bool) -> u32 {
    let init_state = State {
        world: World {
            player: Entity {
                hp: 50,
                damage: 0,
                armor: 0,
            },
            boss: boss.clone(),
            timers: Timers {
                shield: 0,
                poison: 0,
                recharge: 0,
            },
            cur_mana: 500,
        },
        mana_used: 0,
    };
    
    
    let mut frontier = BinaryHeap::new();
    frontier.push(init_state.clone());
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(init_state.world.clone(), init_state.world.clone());
    cost_so_far.insert(init_state.world.clone(), 0);

    while !frontier.is_empty() {
        let current_state = frontier.pop().unwrap();

        if current_state.world.boss.hp <= 0 { return current_state.mana_used }

        for neighbor_state in get_valid_moves(&current_state, hard_mode).iter() {
            let new_cost = neighbor_state.mana_used;
            if !cost_so_far.contains_key(&neighbor_state.world) || new_cost < cost_so_far[&neighbor_state.world] {
                cost_so_far.insert(neighbor_state.world.clone(), neighbor_state.mana_used);
                frontier.push(neighbor_state.clone());
                came_from.insert(neighbor_state.world.clone(), current_state.world.clone());
            }
        }
    }
    u32::MAX
}


fn get_valid_moves(state: &State, hard_mode: bool) -> Vec<State> {
    let mut moves = Vec::new();
    for spell in 0..5 {
        // Check spell reqs
        match spell {
            0 => if state.world.cur_mana < 53 { continue },
            1 => if state.world.cur_mana < 73 { continue },
            2 => if state.world.cur_mana < 113 || state.world.timers.shield > 1 { continue },
            3 => if state.world.cur_mana < 173 || state.world.timers.poison > 1 { continue },
            4 => if state.world.cur_mana < 229 || state.world.timers.recharge > 1 { continue },
            _ => continue,
        }
        let mut cur_state = state.clone();
        // Player turn
        if hard_mode {
            cur_state.world.player.hp -= 1;
            if cur_state.world.player.hp <= 0 { break }
        }
        if cur_state.world.timers.shield > 0 {
            cur_state.world.player.armor = 7;
            cur_state.world.timers.shield -= 1;
        } else {
            cur_state.world.player.armor = 0;
        }
        if cur_state.world.timers.poison > 0 {
            cur_state.world.boss.hp -= 3;
            cur_state.world.timers.poison -= 1;
        }
        if cur_state.world.timers.recharge > 0 {
            cur_state.world.cur_mana += 101;
            cur_state.world.timers.recharge -= 1;
        }
        match spell {
            0 => { // Magic Missile
                cur_state.world.boss.hp -= 4;
                cur_state.mana_used += 53;
                cur_state.world.cur_mana -= 53;
            }
            1 => { // Drain
                cur_state.world.boss.hp -= 2;
                cur_state.world.player.hp += 2;
                cur_state.mana_used += 73;
                cur_state.world.cur_mana -= 73;
            }
            2 => { // Shield
                cur_state.world.timers.shield = 6;
                cur_state.mana_used += 113;
                cur_state.world.cur_mana -= 113;
            }
            3 => { // Poison
                cur_state.world.timers.poison = 6;
                cur_state.mana_used += 173;
                cur_state.world.cur_mana -= 173;
            }
            4 => { // Recharge
                cur_state.world.timers.recharge = 5;
                cur_state.mana_used += 229;
                cur_state.world.cur_mana -= 229;
            }
            _ => { continue }
        }
        // Boss turn
        if cur_state.world.timers.shield > 0 {
            cur_state.world.player.armor = 7;
            cur_state.world.timers.shield -= 1;
        } else {
            cur_state.world.player.armor = 0;
        }
        if cur_state.world.timers.poison > 0 {
            cur_state.world.boss.hp -= 3;
            cur_state.world.timers.poison -= 1;
        }
        if cur_state.world.timers.recharge > 0 {
            cur_state.world.cur_mana += 101;
            cur_state.world.timers.recharge -= 1;
        }

        if cur_state.world.boss.hp > 0 {
            cur_state.world.player.hp -= (cur_state.world.boss.damage - cur_state.world.player.armor).max(1);
        }

        if cur_state.world.player.hp > 0 { moves.push(cur_state) }
    }
    moves
}



#[cfg(test)]
mod test {

    #[test]
    fn test1() {

    }

    #[test]
    fn test2() {

    }
}