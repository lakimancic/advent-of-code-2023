fn diff(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    let mut c: usize = 0;
    for i in 0..s1.len() {
        c += if s1[i] != s2[i] { 1 } else { 0 };
    }
    c
}

fn solve_part(part: &str) -> usize {
    let lines: Vec<Vec<char>> = part.split_ascii_whitespace().map(|line| line.chars().collect::<Vec<char>>()).collect();

    let mut c = 0;
    let mut r = 0;

    let n = lines.len();
    let m = lines[0].len();

    let mut lines_tranposed = vec![vec!['.'; lines.len()]; lines[0].len()];

    for i in 0..lines[0].len() {
        for j in 0..lines.len() {
            lines_tranposed[i][j] = lines[j][i]
        }
    }

    for i in 0..m-1 {
        let mj = std::cmp::min(i + 1, m-i-1);
        let mut eq: usize = 0;
        for j in 0..mj {
            eq += diff(&lines_tranposed[i-j], &lines_tranposed[i+1+j]);
        }

        if eq == 0 {
            c = i + 1;
        }
    }

    for i in 0..n-1 {
        let mj = std::cmp::min(i + 1, n-i-1);
        let mut eq: usize = 0;
        for j in 0..mj {
            eq += diff(&lines[i-j], &lines[i+1+j]);
        }

        if eq == 0 {
            r = i + 1;
        }
    }

    r * 100 + c
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input13.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let parts = txt.split("\n\n");
    let mut sum: usize = 0;

    for part in parts {
        sum += solve_part(part);
    }

    println!("Part 1 solution is: {}", sum);
}