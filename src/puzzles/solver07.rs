use std::collections::HashMap;

fn get_strength(cards: &str) -> u32 {
    let mut count_map: HashMap<char, u8> = HashMap::new();

    for card in cards.chars() {
        *count_map.entry(card).or_insert(0) += 1;
    }

    let mut counts: Vec<u8> = count_map.into_iter().map(|(_, count)| count).collect();
    counts.sort();

    match counts.len() {
        1 => 6,
        2 => if *counts.last().unwrap() == 4u8 {
            5
        } else {
            4
        },
        3 => if *counts.last().unwrap() == 3u8 {
            3
        } else {
            2
        },
        4 => 1,
        _ => 0
    } 
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input07.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let cards1 = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    
    let mut mapped_cards: Vec<(u32,u32)> = txt.lines().map(|line| {
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        let mut st = get_strength(splits[0]);

        for card in splits[0].chars() {
            st = 13 * st + (13 - cards1.into_iter().position(|c| c == card).unwrap() as u32);
        }

        (st, splits[1].parse::<u32>().unwrap())
    }).collect();
    
    mapped_cards.sort();

    let mut winnings: u32 = 0;

    for (i, pair) in mapped_cards.into_iter().enumerate() {
        winnings += (i as u32 + 1) * pair.1;
    }

    println!("Part 1 solution is: {}", winnings);
}