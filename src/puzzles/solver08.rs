use std::collections::HashMap;

pub fn solve() {
    const FILE_PATH: &str = "assets/input08.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines().collect::<Vec<&str>>();

    let seq = lines[0].chars().collect::<Vec<char>>();
    let mut tree: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.into_iter().skip(2) {
        let splits = line.split(" = ").collect::<Vec<&str>>();

        let par = splits[0];
        let childs = splits[1];
        let childs = &childs[1..childs.len()-1].split(", ").collect::<Vec<&str>>();

        let left = childs[0];
        let right = childs[1];

        tree.insert(par, (left, right));
    }

    let mut state = "AAA";
    let mut cnt: usize = 0;

    while state != "ZZZ" {
        if seq[cnt % seq.len()] == 'L' {
            state = tree.get(state).unwrap().0;
        }
        else {
            state = tree.get(state).unwrap().1;
        }
        cnt += 1;
    }

    println!("Part 1 solution is: {}", cnt);
}