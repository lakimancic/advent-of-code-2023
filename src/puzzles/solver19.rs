use std::collections::HashMap;

fn parse_next(hash_map: &HashMap<&str, Vec<&str>>, vals: &HashMap<char, u32>, st: &str) -> String {
    let conds = hash_map.get(st).unwrap();

    for cond in &conds[0..conds.len()-1] {
        let mut splits = cond.split(":");
        let mut cond_chars = splits.next().unwrap().chars();
        let ch = cond_chars.next().unwrap();
        let op = cond_chars.next().unwrap();
        let num = cond_chars.as_str().parse::<u32>().unwrap();
        if op == '>' {
            if *vals.get(&ch).unwrap() > num {
                return splits.next().unwrap().to_string();
            }
        }
        else if op == '<' {
            if *vals.get(&ch).unwrap() < num {
                return splits.next().unwrap().to_string();
            }
        }
    }

    conds.last().unwrap().to_string()
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input19.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut splits = txt.split("\n\n");
    let mut hash_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let re = regex::Regex::new(r"^(\w+)\{((?:[xmas][<>]\d+:\w+,?)+\w+)\}$").unwrap();
    let mut res1 = 0u32;
    
    for line in splits.next().unwrap().lines() {
        let caps = re.captures(line).unwrap();
        hash_map.insert(
            caps.get(1).unwrap().as_str(), 
            caps.get(2).unwrap().as_str().split(",").collect::<Vec<&str>>()
        );
    }

    for line in splits.next().unwrap().lines() {
        let mut vals: HashMap<char, u32> = HashMap::new();

        for part in line[1..line.len()-1].split(",").into_iter() {
            let mut splits = part.split("=");
            let key = splits.next().unwrap().chars().next().unwrap();
            let value = splits.next().unwrap().parse::<u32>().unwrap();
            vals.insert(key, value);
        }

        let mut st: String = "in".to_string();
        while st != "A" && st != "R" {
            st = parse_next(&hash_map, &vals, st.as_str());   
        }

        if st == "A" {
            let sm: u32 = vals.into_iter().map(|x| x.1).sum();
            res1 += sm;
        }
    }

    println!("Part 1 solution is: {}", res1);
}