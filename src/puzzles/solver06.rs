fn read_line_to_vec(line: &str) -> Vec<u32> {
    line.split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn read_line_to_u64(line: &str) -> u64 {
    line.split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input06.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines().collect::<Vec<&str>>();

    let times = read_line_to_vec(lines[0]);
    let dists = read_line_to_vec(lines[1]);
    let mut prod1: u64 = 1;

    for i in 0..times.len() {
        let mut cnt: u64 = 0;
        for j in 1..times[i] {
            if j * (times[i] - j) > dists[i] {
                cnt += 1;
            }
        }
        prod1 *= cnt;
    }

    println!("Part 1 solution is: {}", prod1);

    let time = read_line_to_u64(lines[0]);
    let dist = read_line_to_u64(lines[1]);
    let mut cnt: u64 = 0;

    for i in 1..time {
        if i * (time - i) > dist {
            cnt += 1;
        }
    }

    println!("Part 2 solution is: {}", cnt);
}
