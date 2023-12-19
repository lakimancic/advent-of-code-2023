use regex::Regex;

fn parse_line(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse().unwrap(),
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input01.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines();

    let mut calibration: u32 = 0;
    let mut calibration2: u32 = 0;

    for line in lines {
        let re0 = Regex::new(r"\d").unwrap();
        let re1 = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
        let re2 = Regex::new(r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d").unwrap();

        let caps0: Vec<u32> = re0
            .find_iter(line)
            .map(|x| parse_line(x.as_str()))
            .collect();
        let caps1: Vec<u32> = re1
            .find_iter(line)
            .map(|x| parse_line(x.as_str()))
            .collect();
        let caps2: Vec<u32> = re2
            .find_iter(line.chars().rev().collect::<String>().as_str())
            .map(|x| parse_line(x.as_str().chars().rev().collect::<String>().as_str()))
            .collect();

        calibration += caps0.first().unwrap() * 10 + caps0.last().unwrap();
        calibration2 += caps1.first().unwrap() * 10 + caps2.first().unwrap();
    }

    println!("Part 1 solution is: {}", calibration);
    println!("Part 2 solution is: {}", calibration2);
}
