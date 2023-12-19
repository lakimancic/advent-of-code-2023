use std::collections::HashMap;

fn parse_next(hash_map: &HashMap<&str, Vec<&str>>, vals: &HashMap<char, u32>, st: &str) -> String {
    let conds = hash_map.get(st).unwrap();

    for cond in &conds[0..conds.len() - 1] {
        let mut splits = cond.split(":");
        let mut cond_chars = splits.next().unwrap().chars();
        let ch = cond_chars.next().unwrap();
        let op = cond_chars.next().unwrap();
        let num = cond_chars.as_str().parse::<u32>().unwrap();
        if op == '>' {
            if *vals.get(&ch).unwrap() > num {
                return splits.next().unwrap().to_string();
            }
        } else if op == '<' {
            if *vals.get(&ch).unwrap() < num {
                return splits.next().unwrap().to_string();
            }
        }
    }

    conds.last().unwrap().to_string()
}

struct Range {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Range {
    fn split_by(ch: char, x: u64, r: (u64, u64), rev: bool) -> (u64, u64) {
        let mut nr: (u64, u64) = r;
        if ch == '>' {
            if rev {
                nr.1 = std::cmp::min(r.1, x);
            } else {
                nr.0 = std::cmp::max(r.0, x + 1);
            }
        } else {
            if rev {
                nr.0 = std::cmp::max(r.0, x);
            } else {
                nr.1 = std::cmp::min(r.1, x - 1);
            }
        }
        nr
    }

    fn parse_cond(&self, left: char, op: char, val: u64, rev: bool) -> Self {
        match left {
            'x' => Self {
                x: Range::split_by(op, val, self.x, rev),
                m: self.m,
                a: self.a,
                s: self.s,
            },
            'm' => Self {
                x: self.x,
                m: Range::split_by(op, val, self.m, rev),
                a: self.a,
                s: self.s,
            },
            'a' => Self {
                x: self.x,
                m: self.m,
                a: Range::split_by(op, val, self.a, rev),
                s: self.s,
            },
            's' => Self {
                x: self.x,
                m: self.m,
                a: self.a,
                s: Range::split_by(op, val, self.s, rev),
            },
            _ => Self {
                x: self.x,
                m: self.m,
                a: self.a,
                s: self.s,
            },
        }
    }

    fn calc(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn invalid(&self) -> bool {
        self.x.1 < self.x.0 || self.m.1 < self.m.0 || self.a.1 < self.a.0 || self.s.1 < self.s.0
    }
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
            caps.get(2)
                .unwrap()
                .as_str()
                .split(",")
                .collect::<Vec<&str>>(),
        );
    }

    for line in splits.next().unwrap().lines() {
        let mut vals: HashMap<char, u32> = HashMap::new();

        for part in line[1..line.len() - 1].split(",").into_iter() {
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

    let mut res2 = 0u64;
    let mut queue: Vec<(&str, Range)> = Vec::new();
    queue.push((
        "in",
        Range {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    ));

    while queue.len() > 0 {
        let (st, mut range) = queue.pop().unwrap();

        if range.invalid() {
            continue;
        }

        if st == "A" {
            res2 += range.calc();
        } else if st != "R" {
            let conds = hash_map.get(st).unwrap();

            for cond in &conds[0..conds.len() - 1] {
                let mut splits = cond.split(":");
                let mut cond_chars = splits.next().unwrap().chars();
                let ch = cond_chars.next().unwrap();
                let op = cond_chars.next().unwrap();
                let num = cond_chars.as_str().parse::<u64>().unwrap();
                queue.push((splits.next().unwrap(), range.parse_cond(ch, op, num, false)));
                range = range.parse_cond(ch, op, num, true);
            }

            queue.push((conds[conds.len() - 1], range));
        }
    }

    println!("Part 2 solution is: {}", res2);
}
