use priority_queue::PriorityQueue;
use std::{cmp::Ordering, collections::HashMap, fs, process};
type BotsRelations = HashMap<u32, ((u32, bool), (u32, bool))>;
type BotsContents = PriorityQueue<u32, BotContent>;
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_two_one_answer = solve_both_part(&file_content);
    println!("Part two answer: {:?}", part_two_one_answer);
}

fn solve_both_part(input: &str) -> u32 {
    let (bot_give_bot, mut bot_microships): (BotsRelations, BotsContents) = prepare_data(input);
    let mut outputs: HashMap<u32, u32> = HashMap::new();
    while bot_microships.len() > 1 && bot_microships.peek().unwrap().1.len() == 2 {
        let (bot, bot_content): (u32, BotContent) = bot_microships.pop().unwrap();
        give_microships(
            bot,
            bot_content,
            &bot_give_bot,
            &mut bot_microships,
            &mut outputs,
        );
    }
    outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
}

// In the hashmap the negative values represents outputs and positive values represents bots
// also, the first item is for low value and second is for high value
fn prepare_data(input: &str) -> (BotsRelations, BotsContents) {
    let mut bot_give_bot: BotsRelations = HashMap::new();
    let mut bot_microchips: BotsContents = PriorityQueue::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
        if words.len() == 6 {
            let bot: u32 = words.get(5).unwrap().parse().unwrap();
            let value: u32 = words.get(1).unwrap().parse().unwrap();
            add_microchip(bot, value, &mut bot_microchips);
        } else {
            let bot: u32 = words.get(1).unwrap().parse().unwrap();
            let is_bot_low: bool = *words.get(5).unwrap() == "bot";
            let is_bot_high: bool = *words.get(10).unwrap() == "bot";
            let abs_identifier_low: u32 = words.get(6).unwrap().parse().unwrap();
            let abs_identifier_high: u32 = words.get(11).unwrap().parse().unwrap();
            bot_give_bot.insert(
                bot,
                (
                    (abs_identifier_low, is_bot_low),
                    (abs_identifier_high, is_bot_high),
                ),
            );
        }
    }
    (bot_give_bot, bot_microchips)
}

fn add_microchip(bot: u32, value: u32, bot_microchips: &mut BotsContents) {
    if let Some(content) = bot_microchips.get_priority(&bot) {
        if content.0.is_none() {
            bot_microchips.change_priority(&bot, BotContent(Some(value), None));
        } else {
            bot_microchips.change_priority(&bot, BotContent(content.0, Some(value)));
        }
    } else {
        bot_microchips.push(bot, BotContent(Some(value), None));
    }
}

fn give_microships(
    bot: u32,
    bot_content: BotContent,
    bot_give_bot: &BotsRelations,
    bot_microships: &mut BotsContents,
    outputs: &mut HashMap<u32, u32>,
) {
    if bot_content.0.unwrap() == 61 && bot_content.1.unwrap() == 17 {
        println!("Part one answer: {bot}");
    }
    if bot_content.0.unwrap() < bot_content.1.unwrap() {
        if bot_give_bot.get(&bot).unwrap().0 .1 {
            add_microchip(
                bot_give_bot.get(&bot).unwrap().0 .0,
                bot_content.0.unwrap(),
                bot_microships,
            );
        } else {
            outputs.insert(bot_give_bot.get(&bot).unwrap().0 .0, bot_content.0.unwrap());
        }
        if bot_give_bot.get(&bot).unwrap().1 .1 {
            add_microchip(
                bot_give_bot.get(&bot).unwrap().1 .0,
                bot_content.1.unwrap(),
                bot_microships,
            );
        } else {
            outputs.insert(bot_give_bot.get(&bot).unwrap().1 .0, bot_content.1.unwrap());
        }
    } else {
        if bot_give_bot.get(&bot).unwrap().1 .1 {
            add_microchip(
                bot_give_bot.get(&bot).unwrap().1 .0,
                bot_content.0.unwrap(),
                bot_microships,
            );
        } else {
            outputs.insert(bot_give_bot.get(&bot).unwrap().1 .0, bot_content.0.unwrap());
        }
        if bot_give_bot.get(&bot).unwrap().0 .1 {
            add_microchip(
                bot_give_bot.get(&bot).unwrap().0 .0,
                bot_content.1.unwrap(),
                bot_microships,
            );
        } else {
            outputs.insert(bot_give_bot.get(&bot).unwrap().0 .0, bot_content.1.unwrap());
        }
    }
}

#[derive(Debug)]
struct BotContent(Option<u32>, Option<u32>);
impl BotContent {
    pub fn len(&self) -> usize {
        let mut number_items_self: usize = 0;
        if self.0.is_some() {
            number_items_self += 1;
        }
        if self.1.is_some() {
            number_items_self += 1;
        }
        number_items_self
    }
}
impl Ord for BotContent {
    fn cmp(&self, other: &Self) -> Ordering {
        let number_items_self: usize = self.len();
        let number_items_other: usize = other.len();
        number_items_self.cmp(&number_items_other)
    }
}
impl PartialOrd for BotContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for BotContent {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1)
    }
}
impl Eq for BotContent {}

fn read_input_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error while opening the input file: {:?}", err);
            process::exit(1);
        }
    }
}
