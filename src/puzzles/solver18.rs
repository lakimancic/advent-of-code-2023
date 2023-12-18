fn get_dir(dir: &str) -> (i64, i64) {
    match dir {
        "R" => (0, 1),
        "L" => (0, -1),
        "D" => (1, 0),
        "U" => (-1, 0),
        _ => (0, 0)
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input18.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut prev = (0i64, 0i64);
    let mut point = (0i64, 0i64);
    let mut sum= 0i64;
    let mut pom_sum = 0i64;
    
    for line in txt.lines() {
        let splits = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let d = splits[1].parse::<i64>().unwrap();
        let dir = get_dir(splits[0]);

        point.0 += dir.0 * d;
        point.1 += dir.1 * d;

        sum += point.0 * prev.1 - point.1 * prev.0;
        pom_sum += d;

        prev = point;
    }

    println!("Part 1 solution is: {}", (sum.abs() + pom_sum) / 2 + 1);
}