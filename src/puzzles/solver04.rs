use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input04.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines_count = txt.lines().count();
    let lines = txt.lines();

    let mut sum_of_wins:u64 = 0;
    let mut wins_dp = vec![1; lines_count];

    for (j, line) in lines.enumerate() {
        let splits: Vec<&str> = line.split(":").collect();
        let splits: Vec<&str> = splits[1].split("|").collect();

        let re = Regex::new(r"\d+").unwrap();

        let winnings:Vec<&str> = re.captures_iter(splits[0]).map(|x| x.get(0).unwrap().as_str()).collect();
        let mut num_of_wins: u64 = 0;
        let mut cnt_of_wins: usize = 0;

        for capture in re.captures_iter(splits[1]) {
            if winnings.contains(&capture.get(0).unwrap().as_str()) {
                if num_of_wins == 0 {
                    num_of_wins = 1;
                }
                else {
                    num_of_wins *= 2;
                }
                cnt_of_wins += 1;
            }
        }

        for i in 0..cnt_of_wins {
            wins_dp[j+i+1] += wins_dp[j];
        }

        sum_of_wins += num_of_wins;
    }

    println!("Part 1 solution is: {}", sum_of_wins);

    sum_of_wins = 0;
    for num_of_wins in wins_dp {
        sum_of_wins += num_of_wins;
    }

    println!("Part 2 solution is: {}", sum_of_wins);
}