use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let mut lines = input.lines();

    let boss = Entity {
        hp: lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap(),
        damage: lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap(),
        armor: lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap(),
    };

    println!("{}",get_cheapest_option(&boss));

    println!("{}",get_worst_option(&boss));

}


fn get_cheapest_option(boss: &Entity) -> i32 {
    let item_options = get_item_options();

    let mut min_gold = i32::MAX;

    for items in item_options.iter() {
        let mut player_cost = 0;
        let mut player_damage = 0;
        let mut player_armor = 0;
        for item in items.iter() {
            player_cost += item.cost;
            player_damage += item.damage;
            player_armor += item.armor;
        }

        if player_cost >= min_gold { continue }

        let player = Entity { hp: 100, damage: player_damage, armor: player_armor };

        if battle(&player, boss) {
            min_gold = min_gold.min(player_cost);
            //dbg!(items);
        }
    }

    min_gold
}


fn get_worst_option(boss: &Entity) -> i32 {
    let item_options = get_item_options();

    let mut max_gold = 0;

    for items in item_options.iter() {
        let mut player_cost = 0;
        let mut player_damage = 0;
        let mut player_armor = 0;
        for item in items.iter() {
            player_cost += item.cost;
            player_damage += item.damage;
            player_armor += item.armor;
        }

        if player_cost < max_gold { continue }

        let player = Entity { hp: 100, damage: player_damage, armor: player_armor };

        if !battle(&player, boss) {
            max_gold = max_gold.max(player_cost);
            dbg!(items);
        }
    }

    max_gold
}


fn battle(player: &Entity, boss: &Entity) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();

    loop {
        let tot_damage = (player.damage - boss.armor).max(1);
        boss.hp -= tot_damage;
        if boss.hp <= 0 { break }

        let tot_damage = (boss.damage - player.armor).max(1);
        player.hp -= tot_damage;
        if player.hp <= 0 { break }
    }

    player.hp > 0
}


fn get_item_options() -> Vec<Vec<Item>> {
    let weapons: Vec<Item> = Vec::from([
        (Item {name: "Dagger".to_string(), cost: 8,  damage: 4, armor: 0}),
        (Item {name: "Shortsword".to_string(), cost: 10, damage: 5, armor: 0}),
        (Item {name: "Warhammer".to_string(), cost: 25, damage: 6, armor: 0}),
        (Item {name: "Longsword".to_string(), cost: 40, damage: 7, armor: 0}),
        (Item {name: "Greataxe".to_string(), cost: 74, damage: 8, armor: 0}),
    ]);

    let armor: Vec<Item> = Vec::from([
        (Item {name: "None".to_string(), cost: 0,  damage: 0, armor: 0}),
        (Item {name: "Leather".to_string(), cost: 13, damage: 0, armor: 1}),
        (Item {name: "Chainmail".to_string(), cost: 31, damage: 0, armor: 2}),
        (Item {name: "Splintmail".to_string(), cost: 53, damage: 0, armor: 3}),
        (Item {name: "Bandedmail".to_string(), cost: 75, damage: 0, armor: 4}),
        (Item {name: "Platemail".to_string(), cost: 102, damage: 0, armor: 5}),
    ]);

    let rings: Vec<Item> = Vec::from([
        (Item {name: "None_L".to_string(), cost: 0,  damage: 0, armor: 0}),
        (Item {name: "None_R".to_string(), cost: 0,  damage: 0, armor: 0}),
        (Item {name: "Damage +1".to_string(), cost: 25, damage: 1, armor: 0}),
        (Item {name: "Damage +2".to_string(), cost: 50, damage: 2, armor: 0}),
        (Item {name: "Damage +3".to_string(), cost: 100, damage: 3, armor: 0}),
        (Item {name: "Defense +1".to_string(), cost: 20, damage: 0, armor: 1}),
        (Item {name: "Defense +2".to_string(), cost: 40, damage: 0, armor: 2}),
        (Item {name: "Defense +3".to_string(), cost: 80, damage: 0, armor: 3}),
    ]);

    let mut item_options = Vec::new();

    for cur_weapon in weapons.iter() {
        for cur_armor in armor.iter() {
            for cur_rings in rings.iter().combinations(2) {
                let mut item_vec = Vec::new();
                item_vec.push(cur_weapon.clone());
                item_vec.push(cur_armor.clone());
                for cur_ring in cur_rings {
                    item_vec.push(cur_ring.clone());
                }
                item_options.push(item_vec);
            }
        }
    }

    //dbg!(item_options.len());

    item_options
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Debug)]
struct Entity {
    hp: i32,
    damage: i32,
    armor: i32
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
    }
}