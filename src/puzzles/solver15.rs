fn hash(str: &str) -> u32 {
    let mut h: u32 = 0;
    for ch in str.bytes() {
        h = (h + ch as u32) * 17 % 256;
    }
    h
}

fn find(key: &str, hashmap: &Vec<Vec<(String, u32)>>) -> usize {
    let arr = &hashmap[hash(key) as usize];
    for (i, val) in arr.into_iter().enumerate() {
        if val.0 == key {
            return i;
        }
    }

    usize::MAX
}

fn parse_step(step: &String, hashmap: &mut Vec<Vec<(String, u32)>>) {
    let n = step.len();

    if step.chars().nth(n - 1).unwrap() == '-' {
        let key = &step[0..n - 1];
        let found = find(&key, &hashmap);

        if found < usize::MAX {
            hashmap[hash(key) as usize].remove(found);
        }
    } else {
        let splits = step.split("=").collect::<Vec<&str>>();
        let key = splits[0];
        let value = splits[1].parse::<u32>().unwrap();

        let found = find(key, &hashmap);
        if found < usize::MAX {
            hashmap[hash(key) as usize][found] = (key.to_string(), value);
        } else {
            hashmap[hash(key) as usize].push((key.to_string(), value));
        }
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input15.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let steps = txt.split(",");

    let mut sum1: u32 = 0;
    let mut sum2: usize = 0;
    let mut hashmap: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    for step in steps {
        sum1 += hash(step);

        parse_step(&step.to_string(), &mut hashmap);
    }

    for i in 0..256 {
        for j in 0..hashmap[i].len() {
            sum2 += (i + 1) * (j + 1) * hashmap[i][j].1 as usize;
        }
    }

    println!("Part 1 solution is: {}", sum1);
    println!("Part 2 solution is: {}", sum2);
}
