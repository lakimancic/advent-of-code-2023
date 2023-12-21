use std::collections::{HashMap, VecDeque};

static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn bfs(s: (i64, i64), n: usize, m: usize, steps: usize, mat: &Vec<Vec<char>>) -> usize {
    let mut vis = vec![vec![usize::MAX; m]; n];
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back(s);
    vis[s.0 as usize][s.1 as usize] = 0;
    let mut cnt = 0usize;

    while queue.len() > 0 {
        let (ci, cj) = queue.pop_front().unwrap();
        let dist = vis[ci as usize][cj as usize];

        if dist % 2 == steps % 2 {
            cnt += 1;
        }

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

    cnt
}

fn bfs_inf(s: (i64, i64), n: usize, m: usize, steps: usize, mat: &Vec<Vec<char>>) -> usize {
    let mut vis: HashMap<(i64, i64), usize> = HashMap::new();
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back(s);
    vis.insert((s.0, s.1), 0);
    let mut cnt = 0usize;

    while queue.len() > 0 {
        let (ci, cj) = queue.pop_front().unwrap();
        let dist = vis.get(&(ci, cj)).unwrap().clone();

        if dist % 2 == steps % 2 {
            cnt += 1;
        }

        if dist == steps {
            continue;
        }

        for (di, dj) in DIRS {
            let mut ni = (ci + di) % (n as i64);
            if ni < 0 {
                ni += n as i64;
            }
            let mut nj = (cj + dj) % (m as i64);
            if nj < 0 {
                nj += m as i64;
            }
            if mat[ni as usize][nj as usize] == '#' {
                continue;
            }
            if vis.contains_key(&(ci + di, cj + dj)) {
                continue;
            }
            vis.insert((ci + di, cj + dj), dist + 1);
            queue.push_back((ci + di, cj + dj));
        }
    }

    cnt
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input21.txt";
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

    println!("Part 1 solution is: {}", bfs(s, n, m, 64, &mat));

    let a = bfs_inf(s, n, m, 26501365 % n, &mat);
    let b = bfs_inf(s, n, m, 26501365 % n + n, &mat);
    let c = bfs_inf(s, n, m, 26501365 % n + 2 * n, &mat);
    let k = 26501365 / n;

    let res = k * (b - a + (k - 1) * (a + c - 2 * b) / 2) + a;
    println!("Part 2 solution is: {}", res);
}
