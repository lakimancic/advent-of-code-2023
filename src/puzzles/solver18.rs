static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_dir(dir: &str) -> (i64, i64) {
    match dir {
        "R" => (0, 1),
        "L" => (0, -1),
        "D" => (1, 0),
        "U" => (-1, 0),
        _ => (0, 0),
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input18.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut prev = (0i64, 0i64);
    let mut prev2 = (0i64, 0i64);
    let mut point = (0i64, 0i64);
    let mut point2 = (0i64, 0i64);
    let mut sum = 0i64;
    let mut pom_sum = 0i64;
    let mut sum2 = 0i64;
    let mut pom_sum2 = 0i64;

    for line in txt.lines() {
        let re = regex::Regex::new(r"([UDLR]) (\d+) \(#([0-9a-f]+)\)").unwrap();
        let splits = re.captures(line).unwrap();
        let d = splits[2].parse::<i64>().unwrap();
        let dir = get_dir(&splits[1]);
        let mut hex = i64::from_str_radix(&splits[3], 16).unwrap();
        let dir2 = DIRS[(hex % 16) as usize];
        hex /= 16;

        point.0 += dir.0 * d;
        point.1 += dir.1 * d;

        point2.0 += dir2.0 * hex;
        point2.1 += dir2.1 * hex;

        sum += point.0 * prev.1 - point.1 * prev.0;
        pom_sum += d;
        sum2 += point2.0 * prev2.1 - point2.1 * prev2.0;
        pom_sum2 += hex;

        prev = point;
        prev2 = point2;
    }

    println!("Part 1 solution is: {}", (sum.abs() + pom_sum) / 2 + 1);
    println!("Part 2 solution is: {}", (sum2.abs() + pom_sum2) / 2 + 1);
}
