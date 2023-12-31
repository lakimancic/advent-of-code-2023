static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(i: i64, j: i64, dir: usize, mat: &Vec<Vec<char>>, vis: &mut Vec<Vec<[bool; 4]>>) {
    if mat[i as usize][j as usize] == '.' {
        if i + DIRS[dir].0 < 0 || i + DIRS[dir].0 >= mat.len() as i64 {
            return;
        }
        if j + DIRS[dir].1 < 0 || j + DIRS[dir].1 >= mat[0].len() as i64 {
            return;
        }
        if vis[(i + DIRS[dir].0) as usize][(j + DIRS[dir].1) as usize][dir] {
            return;
        }

        vis[(i + DIRS[dir].0) as usize][(j + DIRS[dir].1) as usize][dir] = true;
        dfs(i + DIRS[dir].0, j + DIRS[dir].1, dir, mat, vis);
    } else if mat[i as usize][j as usize] == '\\' || mat[i as usize][j as usize] == '/' {
        let new_dir: usize;

        if DIRS[dir].0 == 0 {
            if mat[i as usize][j as usize] == '\\' {
                new_dir = (dir + 1) % 4;
            } else {
                new_dir = (dir + 3) % 4;
            }
        } else {
            if mat[i as usize][j as usize] == '\\' {
                new_dir = (dir + 3) % 4;
            } else {
                new_dir = (dir + 1) % 4;
            }
        }

        if i + DIRS[new_dir].0 < 0 || i + DIRS[new_dir].0 >= mat.len() as i64 {
            return;
        }
        if j + DIRS[new_dir].1 < 0 || j + DIRS[new_dir].1 >= mat[0].len() as i64 {
            return;
        }
        if vis[(i + DIRS[new_dir].0) as usize][(j + DIRS[new_dir].1) as usize][new_dir] {
            return;
        }

        vis[(i + DIRS[new_dir].0) as usize][(j + DIRS[new_dir].1) as usize][new_dir] = true;
        dfs(i + DIRS[new_dir].0, j + DIRS[new_dir].1, new_dir, mat, vis);
    } else {
        let new_dir1: usize;
        let new_dir2: usize;

        if DIRS[dir].0 == 0 {
            if mat[i as usize][j as usize] == '-' {
                new_dir1 = dir;
                new_dir2 = dir;
            } else {
                new_dir1 = (dir + 1) % 4;
                new_dir2 = (dir + 3) % 4;
            }
        } else {
            if mat[i as usize][j as usize] == '|' {
                new_dir1 = dir;
                new_dir2 = dir;
            } else {
                new_dir1 = (dir + 1) % 4;
                new_dir2 = (dir + 3) % 4;
            }
        }

        let cond = i + DIRS[new_dir1].0 < 0
            || i + DIRS[new_dir1].0 >= mat.len() as i64
            || j + DIRS[new_dir1].1 < 0
            || j + DIRS[new_dir1].1 >= mat[0].len() as i64
            || vis[(i + DIRS[new_dir1].0) as usize][(j + DIRS[new_dir1].1) as usize][new_dir1];

        if !cond {
            vis[(i + DIRS[new_dir1].0) as usize][(j + DIRS[new_dir1].1) as usize][new_dir1] = true;
            dfs(
                i + DIRS[new_dir1].0,
                j + DIRS[new_dir1].1,
                new_dir1,
                mat,
                vis,
            );
        }

        if new_dir1 == new_dir2 {
            return;
        }

        if i + DIRS[new_dir2].0 < 0 || i + DIRS[new_dir2].0 >= mat.len() as i64 {
            return;
        }
        if j + DIRS[new_dir2].1 < 0 || j + DIRS[new_dir2].1 >= mat[0].len() as i64 {
            return;
        }
        if vis[(i + DIRS[new_dir2].0) as usize][(j + DIRS[new_dir2].1) as usize][new_dir2] {
            return;
        }

        vis[(i + DIRS[new_dir2].0) as usize][(j + DIRS[new_dir2].1) as usize][new_dir2] = true;
        dfs(
            i + DIRS[new_dir2].0,
            j + DIRS[new_dir2].1,
            new_dir2,
            mat,
            vis,
        );
    }
}

fn clear_vis(vis: &mut Vec<Vec<[bool; 4]>>) {
    for i in vis {
        for j in i {
            for k in j {
                *k = false;
            }
        }
    }
}

fn count_vis(vis: &Vec<Vec<[bool; 4]>>) -> usize {
    let mut cnt = 0usize;
    for i in vis {
        for j in i {
            let mut f = false;
            for k in j {
                f = f || *k;
            }
            cnt += f as usize;
        }
    }
    cnt
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input16.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mat = txt
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n = mat.len();
    let m = mat[0].len();

    let mut vis = vec![vec![[false; 4]; m]; n];
    vis[0][0][0] = true;
    dfs(0, 0, 0, &mat, &mut vis);
    let cnt: usize = count_vis(&vis);
    let mut max_cnt: usize = cnt;
    let mut s = (0i64, 1i64);

    for i in 1..5usize {
        let mn = if i % 2 == 1 { m } else { n };
        for j in 1..mn {
            clear_vis(&mut vis);
            vis[s.0 as usize][s.1 as usize][i % 4] = true;
            dfs(s.0, s.1, i % 4, &mat, &mut vis);
            max_cnt = std::cmp::max(max_cnt, count_vis(&vis));
            if j == mn - 1 {
                clear_vis(&mut vis);
                vis[s.0 as usize][s.1 as usize][(i + 1) % 4] = true;
                dfs(s.0, s.1, (i + 1) % 4, &mat, &mut vis);
                max_cnt = std::cmp::max(max_cnt, count_vis(&vis));
                s.0 += DIRS[i % 4].0;
                s.1 += DIRS[i % 4].1;
            } else {
                s.0 += DIRS[i - 1].0;
                s.1 += DIRS[i - 1].1;
            }
        }
    }

    println!("Part 1 solution is: {}", cnt);
    println!("Part 2 solution is: {}", max_cnt);
}
