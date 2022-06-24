use enum_iterator::all;
use game::*;
use priority_queue::PriorityQueue;
use std::{fs, process};

const start_life: u32 = 50;
const start_mana: u32 = 500;

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> i32 {
    let mut initial_boss: Boss = read_boss_data(input);
    let mut initial_player: Player = Player::new(start_life, start_mana, 0);
    let avaible_casts: Vec<Cast> = avaible_casts_with_mana(initial_player.get_mana(), &Vec::new());
    let mut less_spent_mana: PriorityQueue<(Player, Boss, Vec<(Effects, u32)>, Cast), i32> =
        PriorityQueue::new();
    initial_player.deal_damage(1);
    for cast in avaible_casts {
        less_spent_mana.push(
            (
                initial_player.clone(),
                initial_boss.clone(),
                Vec::new(),
                cast,
            ),
            -(cast.get_cost() as i32),
        );
    }
    loop {
        let ((player, boss, effects, next_cast), mana_spent) = less_spent_mana.pop().unwrap();
        let did_won = next_round(
            player.clone(),
            boss.clone(),
            effects.clone(),
            next_cast,
            &mut less_spent_mana,
            mana_spent.abs() as u32,
            false,
        );
        if did_won {
            return mana_spent.abs();
        }
    }
}

fn solve_part_two(input: &str) -> i32 {
    let mut initial_boss: Boss = read_boss_data(input);
    let mut initial_player: Player = Player::new(start_life, start_mana, 0);
    let avaible_casts: Vec<Cast> = avaible_casts_with_mana(initial_player.get_mana(), &Vec::new());
    let mut less_spent_mana: PriorityQueue<(Player, Boss, Vec<(Effects, u32)>, Cast), i32> =
        PriorityQueue::new();
    for cast in avaible_casts {
        less_spent_mana.push(
            (
                initial_player.clone(),
                initial_boss.clone(),
                Vec::new(),
                cast,
            ),
            -(cast.get_cost() as i32),
        );
    }
    loop {
        let ((player, boss, effects, next_cast), mana_spent) = less_spent_mana.pop().unwrap();
        let did_won = next_round(
            player.clone(),
            boss.clone(),
            effects.clone(),
            next_cast,
            &mut less_spent_mana,
            mana_spent.abs() as u32,
            true,
        );
        if did_won {
            return mana_spent.abs();
        }
    }
}

fn next_round(
    mut player: Player,
    mut boss: Boss,
    mut effects: Vec<(Effects, u32) /*effect and round left*/>,
    next_cast: Cast,
    less_spent_mana: &mut PriorityQueue<(Player, Boss, Vec<(Effects, u32)>, Cast), i32>,
    mana_spent: u32,
    part_two: bool,
) -> bool {
    player.remove_mana(next_cast.get_cost());
    match next_cast {
        Cast::Direct(cast) => {
            boss.deal_damage(cast.get_damage());
            player.heal(cast.get_heal());
            if !boss.is_alive() {
                return true;
            }
        }
        Cast::Effect(cast) => {
            effects.push((cast, cast.get_initial_time()));
        }
    }

    ///////////////////
    // boss turn
    for (effect, round_left) in effects.iter_mut() {
        boss.deal_damage(effect.get_continuous_delta_damage());
        if !boss.is_alive() {
            return true;
        }
        player.add_mana(effect.get_continuous_delta_mana());
        if effect.get_initial_time() == *round_left {
            player.add_armor(effect.get_delta_armor());
        }
        *round_left -= 1;
        if *round_left <= 0 {
            player.remove_armor(effect.get_delta_armor());
        }
    }
    effects = effects
        .into_iter()
        .filter(|&x| x.1 > 0)
        .collect::<Vec<(Effects, u32)>>();
    player.deal_damage(boss.get_damage());
    if !player.is_alive() {
        return false;
    }

    /////////////////////////
    // player turn
    if part_two {
        player.deal_damage(1);
    }
    for (effect, round_left) in effects.iter_mut() {
        boss.deal_damage(effect.get_continuous_delta_damage());
        if !boss.is_alive() {
            return true;
        }
        player.add_mana(effect.get_continuous_delta_mana());
        if effect.get_initial_time() == *round_left {
            player.add_armor(effect.get_delta_armor());
        }
        *round_left -= 1;
        if *round_left <= 0 {
            player.remove_armor(effect.get_delta_armor());
        }
    }
    effects = effects
        .into_iter()
        .filter(|&x| x.1 > 0)
        .collect::<Vec<(Effects, u32)>>();
    let avaible_casts: Vec<Cast> = avaible_casts_with_mana(player.get_mana(), &effects);
    if avaible_casts.is_empty() {
        return false;
    }
    for avaible_cast in avaible_casts {
        less_spent_mana.push(
            (player.clone(), boss.clone(), effects.clone(), avaible_cast),
            -((mana_spent + avaible_cast.get_cost()) as i32),
        );
    }
    return false;
}


fn avaible_casts_with_mana(mana: u32, effects: &Vec<(Effects, u32)>) -> Vec<Cast> {
    let mut avaible_casts: Vec<Cast> = Vec::new();
    'first: for cast in all::<Cast>() {
        if cast.get_cost() <= mana {
            if let Cast::Effect(cast_eff) = cast {
                for effect in effects {
                    if cast_eff == effect.0 {
                        continue 'first;
                    }
                }
                avaible_casts.push(Cast::Effect(cast_eff));
            } else {
                avaible_casts.push(cast);
            }
        }
    }
    return avaible_casts;
}

fn read_boss_data(input: &str) -> Boss {
    let life_damage: Vec<u32> = input
        .lines()
        .map(|x| {
            x.split(": ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<u32>>();
    Boss::new(life_damage[0], life_damage[1])
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
    #[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Player {
        life: i32,
        mana: i32,
        armor: i32,
    }
    impl Player {
        pub fn new(starting_life: u32, starting_mana: u32, starting_armor: u32) -> Player {
            Player {
                life: starting_life as i32,
                mana: starting_mana as i32,
                armor: starting_armor as i32,
            }
        }
        pub fn deal_damage(&mut self, damage: u32) {
            if (*self).armor >= damage as i32 {
                (*self).life -= 1;
            } else {
                (*self).life -= damage as i32 - (*self).armor
            }
            if (*self).life < 0 {
                (*self).life = 0;
            }
        }
        pub fn add_armor(&mut self, delta_armor: u32) {
            (*self).armor += delta_armor as i32;
        }
        pub fn remove_armor(&mut self, delta_armor: u32) {
            (*self).armor -= delta_armor as i32;
            if (*self).armor < 0 {
                (*self).armor = 0;
            }
        }
        pub fn heal(&mut self, healing: u32) {
            (*self).life += healing as i32;
        }
        pub fn add_mana(&mut self, delta_mana: u32) {
            (*self).mana += delta_mana as i32;
        }
        pub fn remove_mana(&mut self, delta_mana: u32) {
            (*self).mana -= delta_mana as i32;
            if (*self).mana < 0 {
                (*self).mana = 0;
            }
        }
        pub fn get_mana(&self) -> u32 {
            (*self).mana as u32
        }
        pub fn is_alive(&self) -> bool {
            (*self).life > 0
        }
    }
    #[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Boss {
        life: i32,
        damage: u32,
    }
    impl Boss {
        pub fn new(starting_life: u32, damage: u32) -> Boss {
            Boss {
                life: starting_life as i32,
                damage: damage,
            }
        }
        pub fn deal_damage(&mut self, damage: u32) {
            (*self).life -= damage as i32;
            if (*self).life < 0 {
                (*self).life = 0;
            }
        }
        pub fn is_alive(&self) -> bool {
            (*self).life > 0
        }
        pub fn get_damage(&self) -> u32 {
            (*self).damage
        }
    }

    #[derive(Hash, Clone, Copy, Sequence, Debug, PartialEq, PartialOrd, Ord, Eq)]
    pub enum Cast {
        Direct(Directs),
        Effect(Effects),
    }
    impl Cast {
        pub fn get_cost(&self) -> u32 {
            match self {
                Cast::Direct(cast) => cast.get_cost(),
                Cast::Effect(cast) => cast.get_cost(),
            }
        }
    }

    #[derive(Hash, Clone, Copy, Sequence, Debug, PartialEq, PartialOrd, Ord, Eq)]
    pub enum Directs {
        MagicMissile,
        Drain,
    }
    #[derive(Hash, Clone, Copy, Sequence, Debug, PartialEq, PartialOrd, Ord, Eq)]
    pub enum Effects {
        Shield,
        Poison,
        Recharge,
    }

    struct Effect {
        cost: u32,
        initial_time: u32,
        delta_armor: u32,
        continuous_delta_damage: u32,
        continuous_delta_mana: u32,
    }
    struct Direct {
        cost: u32,
        damage: u32,
        heal: u32,
    }

    impl Direct {
        const MAGIC_MISSILE: Direct = Direct {
            cost: 53,
            damage: 4,
            heal: 0,
        };
        const DRAIN_STATS: Direct = Direct {
            cost: 73,
            damage: 2,
            heal: 2,
        };
    }

    impl Directs {
        pub fn get_cost(&self) -> u32 {
            match *self {
                Directs::MagicMissile => Direct::MAGIC_MISSILE.cost,
                Directs::Drain => Direct::DRAIN_STATS.cost,
            }
        }
        pub fn get_damage(&self) -> u32 {
            match *self {
                Directs::MagicMissile => Direct::MAGIC_MISSILE.damage,
                Directs::Drain => Direct::DRAIN_STATS.damage,
            }
        }
        pub fn get_heal(&self) -> u32 {
            match *self {
                Directs::MagicMissile => Direct::MAGIC_MISSILE.heal,
                Directs::Drain => Direct::DRAIN_STATS.heal,
            }
        }
    }

    impl Effect {
        const SHIELD_STATS: Effect = Effect {
            cost: 113,
            initial_time: 6,
            delta_armor: 7,
            continuous_delta_damage: 0,
            continuous_delta_mana: 0,
        };
        const POISON_STATS: Effect = Effect {
            cost: 173,
            initial_time: 6,
            delta_armor: 0,
            continuous_delta_damage: 3,
            continuous_delta_mana: 0,
        };
        const RECHARGE_STATS: Effect = Effect {
            cost: 229,
            initial_time: 5,
            delta_armor: 0,
            continuous_delta_damage: 0,
            continuous_delta_mana: 101,
        };
    }

    impl Effects {
        pub fn get_cost(&self) -> u32 {
            match *self {
                Effects::Shield => Effect::SHIELD_STATS.cost,
                Effects::Poison => Effect::POISON_STATS.cost,
                Effects::Recharge => Effect::RECHARGE_STATS.cost,
            }
        }
        pub fn get_initial_time(&self) -> u32 {
            match *self {
                Effects::Shield => Effect::SHIELD_STATS.initial_time,
                Effects::Poison => Effect::POISON_STATS.initial_time,
                Effects::Recharge => Effect::RECHARGE_STATS.initial_time,
            }
        }
        pub fn get_delta_armor(&self) -> u32 {
            match *self {
                Effects::Shield => Effect::SHIELD_STATS.delta_armor,
                Effects::Poison => Effect::POISON_STATS.delta_armor,
                Effects::Recharge => Effect::RECHARGE_STATS.delta_armor,
            }
        }
        pub fn get_continuous_delta_damage(&self) -> u32 {
            match *self {
                Effects::Shield => Effect::SHIELD_STATS.continuous_delta_damage,
                Effects::Poison => Effect::POISON_STATS.continuous_delta_damage,
                Effects::Recharge => Effect::RECHARGE_STATS.continuous_delta_damage,
            }
        }
        pub fn get_continuous_delta_mana(&self) -> u32 {
            match *self {
                Effects::Shield => Effect::SHIELD_STATS.continuous_delta_mana,
                Effects::Poison => Effect::POISON_STATS.continuous_delta_mana,
                Effects::Recharge => Effect::RECHARGE_STATS.continuous_delta_mana,
            }
        }
    }
}
