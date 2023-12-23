static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(mat: &Vec<Vec<char>>, n: usize, m: usize, vis: &mut Vec<Vec<bool>>, pos: (i64, i64)) -> i64 {
    if pos.0 as usize == n - 1 && pos.1 as usize == m - 2 {
        return 0;
    }
    if mat[pos.0 as usize][pos.1 as usize] != '.' {
        let new_pos = match mat[pos.0 as usize][pos.1 as usize] {
            '>' => (pos.0, pos.1 + 1),
            '<' => (pos.0, pos.1 - 1),
            'v' => (pos.0 + 1, pos.1),
            '^' => (pos.0 - 1, pos.1),
            _ => (0, 0),
        };
        if vis[new_pos.0 as usize][new_pos.1 as usize] {
            return -1;
        }
        vis[new_pos.0 as usize][new_pos.1 as usize] = true;
        let res = dfs(mat, n, m, vis, new_pos);
        vis[new_pos.0 as usize][new_pos.1 as usize] = false;
        return if res != -1 { res + 1 } else { res };
    }
    let mut res = -1i64;
    for (di, dj) in DIRS {
        let np = (pos.0 + di, pos.1 + dj);
        if np.0 < 0 || np.0 >= n as i64 {
            continue;
        }
        if np.1 < 0 || np.1 >= m as i64 {
            continue;
        }
        if mat[np.0 as usize][np.1 as usize] == '#' {
            continue;
        }
        if vis[np.0 as usize][np.1 as usize] {
            continue;
        }
        vis[np.0 as usize][np.1 as usize] = true;
        let new_res = dfs(mat, n, m, vis, np);
        if new_res != -1 {
            res = std::cmp::max(res, new_res + 1);
        }
        vis[np.0 as usize][np.1 as usize] = false;
    }

    res
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input23.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mat = txt
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = mat.len();
    let m = mat[0].len();

    let mut vis = vec![vec![false; m]; n];
    vis[0][1] = true;
    println!("Part 1 solution is: {}", dfs(&mat, n, m, &mut vis, (0, 1)));
}
