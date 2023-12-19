use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    let mut ma = a;
    let mut mb = b;

    while mb > 0 {
        let tmp = mb;
        mb = ma % mb;
        ma = tmp;
    }

    ma
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input08.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines().collect::<Vec<&str>>();

    let seq = lines[0].chars().collect::<Vec<char>>();
    let mut tree: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut states: Vec<&str> = Vec::new();

    for line in lines.into_iter().skip(2) {
        let splits = line.split(" = ").collect::<Vec<&str>>();

        let par = splits[0];
        let childs = splits[1];
        let childs = &childs[1..childs.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();

        let left = childs[0];
        let right = childs[1];

        tree.insert(par, (left, right));

        if par.ends_with("A") {
            states.push(par);
        }
    }

    let mut state = "AAA";
    let mut cnt: usize = 0;

    while state != "ZZZ" {
        if seq[cnt % seq.len()] == 'L' {
            state = tree.get(state).unwrap().0;
        } else {
            state = tree.get(state).unwrap().1;
        }
        cnt += 1;
    }

    let mut cnt2: usize = 1;

    for state in states {
        let mut new_state = state;

        cnt = 0;

        while !new_state.ends_with("Z") {
            if seq[cnt % seq.len()] == 'L' {
                new_state = tree.get(new_state).unwrap().0;
            } else {
                new_state = tree.get(new_state).unwrap().1;
            }
            cnt += 1;
        }

        cnt2 = lcm(cnt, cnt2);
    }

    println!("Part 1 solution is: {}", cnt);
    println!("Part 2 solution is: {}", cnt2);
}
