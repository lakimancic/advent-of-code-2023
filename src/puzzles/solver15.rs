fn hash(str: &str) -> u32 {
    let mut h: u32 = 0;
    for ch in str.bytes() {
        h = (h + ch as u32) * 17 % 256;
    }
    h
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input15.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let steps = txt.split(",");

    let mut sum1: u32 = 0;

    for step in steps {
        sum1 += hash(step);
    }

    println!("Part 1 solution is: {}", sum1);
}