use enum_iterator::all;
use game::*;
use std::{fs, process};

const START_LIFE: u32 = 100;
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u32 {
    let (boss, cost_sorted_player_list) = prepare_data(input);
    for player in cost_sorted_player_list {
        if does_player_win(&player, &boss) {
            return player.get_cost();
        }
    }
    return 0;
}

fn solve_part_two(input: &str) -> u32 {
    let (boss, cost_sorted_player_list) = prepare_data(input);
    for player in cost_sorted_player_list.iter().rev() {
        if !does_player_win(&player, &boss) {
            return player.get_cost();
        }
    }
    return 0;
}

fn does_player_win(player: &Player, boss: &Boss) -> bool {
    let player_actual_damage = {
        if boss.get_armor() >= player.get_damage() {
            1
        } else {
            player.get_damage() - boss.get_armor()
        }
    };
    let boss_actual_damage = {
        if player.get_armor() >= boss.get_damage() {
            1
        } else {
            boss.get_damage() - player.get_armor()
        }
    };
    if boss.get_life() / player_actual_damage <= player.get_life() / boss_actual_damage {
        return true;
    }
    return false;
}

fn recursive_combination<T: Clone + Copy + PartialEq>(
    items_list: &Vec<T>,
    level: u32,
    next_start_index: usize,
    combination_list: Vec<T>,
    combinations_list: &mut Vec<Vec<T>>,
) {
    if level == 0 {
        if !combinations_list.contains(&combination_list) {
            combinations_list.push(combination_list);
        }
        return;
    } else {
        for i in next_start_index..items_list.len() {
            let mut cloned_content_list = combination_list.clone();
            cloned_content_list.push(items_list[i]);
            recursive_combination(
                items_list,
                level - 1,
                i + 1,
                cloned_content_list,
                combinations_list,
            );
        }
    }
}

fn prepare_data(input: &str) -> (Boss, Vec<Player>) {
    let life_damage_armor: Vec<u32> = input
        .lines()
        .map(|x| {
            x.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap()
        })
        .collect(); //0: life, 1: damage, 2: armor
    let boss: Boss = Boss::new(
        life_damage_armor[1],
        life_damage_armor[2],
        life_damage_armor[0],
    );
    let all_weapons: Vec<Weapons> = all::<Weapons>().collect::<Vec<Weapons>>();
    let all_armors: Vec<Armors> = all::<Armors>().collect::<Vec<Armors>>();
    let mut all_rings: Vec<Rings> = all::<Rings>().collect::<Vec<Rings>>();
    all_rings.push(Rings::None);
    let mut all_weapons_combinations: Vec<Vec<Weapons>> = Vec::new();
    let mut all_armors_combinations: Vec<Vec<Armors>> = Vec::new();
    let mut all_rings_combinations: Vec<Vec<Rings>> = Vec::new();
    recursive_combination(
        &all_weapons,
        1,
        0,
        Vec::new(),
        &mut all_weapons_combinations,
    );
    recursive_combination(&all_armors, 1, 0, Vec::new(), &mut all_armors_combinations);
    recursive_combination(&all_rings, 2, 0, Vec::new(), &mut all_rings_combinations);
    let mut all_possible_players: Vec<Player> = Vec::new();
    for weapon in all_weapons_combinations {
        for armor in &all_armors_combinations {
            for rings in &all_rings_combinations {
                all_possible_players.push(Player::new(weapon[0], armor[0], [rings[0], rings[1]]));
            }
        }
    }
    all_possible_players.sort_by(|a, b| a.get_cost().cmp(&b.get_cost()));
    (boss, all_possible_players)
}

fn read_input_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(content) => return content,
        Err(err) => {
            eprintln!("Error while opening the input file: {:?}", err);
            process::exit(1);
        }
    };
}

mod game {
    use enum_iterator::Sequence;
    pub struct Player {
        cost_value: u32,
        damage_value: u32,
        armor_value: u32,
        life: u32,
    }

    impl Player {
        pub fn new(weapon: Weapons, armor: Armors, rings: [Rings; 2]) -> Player {
            let cost_value: u32 =
                weapon.get_cost() + armor.get_cost() + rings[0].get_cost() + rings[1].get_cost();
            let damage_value: u32 = weapon.get_damage()
                + armor.get_damage()
                + rings[0].get_damage()
                + rings[1].get_damage();
            let armor_value: u32 = weapon.get_armor()
                + armor.get_armor()
                + rings[0].get_armor()
                + rings[1].get_armor();
            Player {
                cost_value: cost_value,
                damage_value: damage_value,
                armor_value: armor_value,
                life: crate::START_LIFE,
            }
        }

        pub fn get_cost(&self) -> u32 {
            return (*self).cost_value;
        }

        pub fn get_damage(&self) -> u32 {
            return (*self).damage_value;
        }

        pub fn get_armor(&self) -> u32 {
            return (*self).armor_value;
        }

        pub fn get_life(&self) -> u32 {
            return (*self).life;
        }
    }

    pub struct Boss {
        damage_value: u32,
        armor_value: u32,
        life: u32,
    }

    impl Boss {
        pub fn new(damage_value: u32, armor_value: u32, life: u32) -> Boss {
            Boss {
                damage_value: damage_value,
                armor_value: armor_value,
                life: life,
            }
        }

        pub fn get_damage(&self) -> u32 {
            return (*self).damage_value;
        }

        pub fn get_armor(&self) -> u32 {
            return (*self).armor_value;
        }

        pub fn get_life(&self) -> u32 {
            return (*self).life;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Sequence)]
    pub enum Weapons {
        Dagger,
        Shortsword,
        Warhammer,
        Longsword,
        Greataxe,
    }

    impl Weapons {
        const DAGGER_STATS: (u32, u32, u32) = (8, 4, 0);
        const SHORTSWORD_STATS: (u32, u32, u32) = (10, 5, 0);
        const WARHAMMER_STATS: (u32, u32, u32) = (25, 6, 0);
        const LONGSWORD_STATS: (u32, u32, u32) = (40, 7, 0);
        const GREATAXE_STATS: (u32, u32, u32) = (75, 8, 0);
        pub fn get_cost(&self) -> u32 {
            match *self {
                Weapons::Dagger => return Weapons::DAGGER_STATS.0,
                Weapons::Shortsword => return Weapons::SHORTSWORD_STATS.0,
                Weapons::Warhammer => return Weapons::WARHAMMER_STATS.0,
                Weapons::Longsword => return Weapons::LONGSWORD_STATS.0,
                Weapons::Greataxe => return Weapons::GREATAXE_STATS.0,
            }
        }
        pub fn get_damage(&self) -> u32 {
            match *self {
                Weapons::Dagger => return Weapons::DAGGER_STATS.1,
                Weapons::Shortsword => return Weapons::SHORTSWORD_STATS.1,
                Weapons::Warhammer => return Weapons::WARHAMMER_STATS.1,
                Weapons::Longsword => return Weapons::LONGSWORD_STATS.1,
                Weapons::Greataxe => return Weapons::GREATAXE_STATS.1,
            }
        }
        pub fn get_armor(&self) -> u32 {
            match *self {
                Weapons::Dagger => return Weapons::DAGGER_STATS.2,
                Weapons::Shortsword => return Weapons::SHORTSWORD_STATS.2,
                Weapons::Warhammer => return Weapons::WARHAMMER_STATS.2,
                Weapons::Longsword => return Weapons::LONGSWORD_STATS.2,
                Weapons::Greataxe => return Weapons::GREATAXE_STATS.2,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Sequence)]
    pub enum Armors {
        Leather,
        Chainmail,
        Splintmail,
        Bandedmail,
        Platemail,
        None,
    }

    impl Armors {
        const LEATHER_STATS: (u32, u32, u32) = (13, 0, 1);
        const CHAINMAIL_STATS: (u32, u32, u32) = (31, 0, 2);
        const SPLINTMAIL_STATS: (u32, u32, u32) = (53, 0, 3);
        const BANDEDMAIL_STATS: (u32, u32, u32) = (75, 0, 4);
        const PLATEMAIL_STATS: (u32, u32, u32) = (102, 0, 5);
        pub fn get_cost(&self) -> u32 {
            match *self {
                Armors::Leather => return Armors::LEATHER_STATS.0,
                Armors::Chainmail => return Armors::CHAINMAIL_STATS.0,
                Armors::Splintmail => return Armors::SPLINTMAIL_STATS.0,
                Armors::Bandedmail => return Armors::BANDEDMAIL_STATS.0,
                Armors::Platemail => return Armors::PLATEMAIL_STATS.0,
                Armors::None => return 0,
            }
        }
        pub fn get_damage(&self) -> u32 {
            match *self {
                Armors::Leather => return Armors::LEATHER_STATS.1,
                Armors::Chainmail => return Armors::CHAINMAIL_STATS.1,
                Armors::Splintmail => return Armors::SPLINTMAIL_STATS.1,
                Armors::Bandedmail => return Armors::BANDEDMAIL_STATS.1,
                Armors::Platemail => return Armors::PLATEMAIL_STATS.1,
                Armors::None => return 0,
            }
        }
        pub fn get_armor(&self) -> u32 {
            match *self {
                Armors::Leather => return Armors::LEATHER_STATS.2,
                Armors::Chainmail => return Armors::CHAINMAIL_STATS.2,
                Armors::Splintmail => return Armors::SPLINTMAIL_STATS.2,
                Armors::Bandedmail => return Armors::BANDEDMAIL_STATS.2,
                Armors::Platemail => return Armors::PLATEMAIL_STATS.2,
                Armors::None => return 0,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Sequence)]
    pub enum Rings {
        Damage1,
        Damage2,
        Damage3,
        Defense1,
        Defense2,
        Defense3,
        None,
    }

    impl Rings {
        const DAMAGE1_STATS: (u32, u32, u32) = (25, 1, 0);
        const DAMAGE2_STATS: (u32, u32, u32) = (50, 2, 0);
        const DAMAGE3_STATS: (u32, u32, u32) = (100, 3, 0);
        const DEFENSE1_STATS: (u32, u32, u32) = (20, 0, 1);
        const DEFENSE2_STATS: (u32, u32, u32) = (40, 0, 2);
        const DEFENSE3_STATS: (u32, u32, u32) = (80, 0, 3);
        pub fn get_cost(&self) -> u32 {
            match *self {
                Rings::Damage1 => return Rings::DAMAGE1_STATS.0,
                Rings::Damage2 => return Rings::DAMAGE2_STATS.0,
                Rings::Damage3 => return Rings::DAMAGE3_STATS.0,
                Rings::Defense1 => return Rings::DEFENSE1_STATS.0,
                Rings::Defense2 => return Rings::DEFENSE2_STATS.0,
                Rings::Defense3 => return Rings::DEFENSE3_STATS.0,
                Rings::None => return 0,
            }
        }
        pub fn get_damage(&self) -> u32 {
            match *self {
                Rings::Damage1 => return Rings::DAMAGE1_STATS.1,
                Rings::Damage2 => return Rings::DAMAGE2_STATS.1,
                Rings::Damage3 => return Rings::DAMAGE3_STATS.1,
                Rings::Defense1 => return Rings::DEFENSE1_STATS.1,
                Rings::Defense2 => return Rings::DEFENSE2_STATS.1,
                Rings::Defense3 => return Rings::DEFENSE3_STATS.1,
                Rings::None => return 0,
            }
        }
        pub fn get_armor(&self) -> u32 {
            match *self {
                Rings::Damage1 => return Rings::DAMAGE1_STATS.2,
                Rings::Damage2 => return Rings::DAMAGE2_STATS.2,
                Rings::Damage3 => return Rings::DAMAGE3_STATS.2,
                Rings::Defense1 => return Rings::DEFENSE1_STATS.2,
                Rings::Defense2 => return Rings::DEFENSE2_STATS.2,
                Rings::Defense3 => return Rings::DEFENSE3_STATS.2,
                Rings::None => return 0,
            }
        }
    }
}
