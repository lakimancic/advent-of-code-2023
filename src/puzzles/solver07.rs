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
    let cards2 = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    
    let mut mapped_cards1: Vec<(u32,u32)> = txt.lines().map(|line| {
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        let mut st = get_strength(splits[0]);

        for card in splits[0].chars() {
            st = 13 * st + (13 - cards1.into_iter().position(|c| c == card).unwrap() as u32);
        }

        (st, splits[1].parse::<u32>().unwrap())
    }).collect();

    let mut mapped_cards2: Vec<(u32,u32)> = txt.lines().map(|line| {
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        
        let mut max_st = 0;

        for j in 0..cards2.len()-1 {
            let mut st = get_strength(splits[0].replace('J', cards2[j].to_string().as_str()).as_str());

            for card in splits[0].chars() {
                st = 13 * st + (13 - cards2.into_iter().position(|c| c == card).unwrap() as u32);
            }

            max_st = std::cmp::max(max_st, st);
        }

        (max_st, splits[1].parse::<u32>().unwrap())
    }).collect();
    
    mapped_cards1.sort();
    mapped_cards2.sort();

    let mut winnings1: u32 = 0;
    let mut winnings2: u32 = 0;

    for (i, pair) in mapped_cards1.into_iter().enumerate() {
        winnings1 += (i as u32 + 1) * pair.1;
    }

    for (i, pair) in mapped_cards2.into_iter().enumerate() {
        winnings2 += (i as u32 + 1) * pair.1;
    }

    println!("Part 1 solution is: {}", winnings1);
    println!("Part 2 solution is: {}", winnings2);
}