use std::collections::VecDeque;

static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn solve() {
    const FILE_PATH: &str = "assets/input00.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mat = txt
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = mat.len();
    let m = mat[0].len();

    let mut s = (0i64, 0i64);

    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 'S' {
                s.0 = i as i64;
                s.1 = j as i64;
            }
        }
    }

    let steps = 64;
    let mut vis = vec![vec![usize::MAX; m]; n];
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back(s);
    vis[s.0 as usize][s.1 as usize] = 0;

    while queue.len() > 0 {
        let (ci, cj) = queue.pop_front().unwrap();
        let dist = vis[ci as usize][cj as usize];

        if dist == steps {
            continue;
        }

        for (di, dj) in DIRS {
            if ci + di < 0 || ci + di >= (n as i64) {
                continue;
            }
            if cj + dj < 0 || cj + dj >= (m as i64) {
                continue;
            }
            if mat[(ci + di) as usize][(cj + dj) as usize] == '#' {
                continue;
            }
            if vis[(ci + di) as usize][(cj + dj) as usize] != usize::MAX {
                continue;
            }
            vis[(ci + di) as usize][(cj + dj) as usize] = dist + 1;
            queue.push_back((ci + di, cj + dj));
        }
    }

    let mut cnt = 0usize;

    for row in vis {
        for cell in row {
            if cell != usize::MAX && cell % 2 == steps % 2 {
                cnt += 1;
            }
        }
    }

    println!("Part 1 solution is: {}", cnt);
}
