fn line_to_vec32(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn calc_sum(nums: &mut Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    let mut n = nums.len();
    let mut is_loop = true;

    while is_loop {
        is_loop = false;

        for i in 0..n-1 {
            nums[i] = nums[i+1] - nums[i];
            if nums[i] != 0 {
                is_loop = true;
            }
        }

        n -= 1;
        sum += nums[n];
    }

    sum
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input09.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;

    for line in txt.lines() {
        let mut nums = line_to_vec32(line);
        let mut nums2 = line_to_vec32(line);
        nums2.reverse();

        sum1 += calc_sum(&mut nums);
        sum2 += calc_sum(&mut nums2);
    }

    println!("Part 1 solution is: {}", sum1);
    println!("Part 2 solution is: {}", sum2);
}