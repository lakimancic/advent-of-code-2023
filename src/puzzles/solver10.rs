use std::collections::HashMap;

fn check_dirs(dir1: (i32, i32), dir2: &Vec<(i32, i32)>) -> bool {
    for (di, dj) in dir2 {
        if di + dir1.0 == 0 && dj + dir1.1 == 0 {
            return true;
        }
    }
    false
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input10.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut mat: Vec<Vec<char>> = txt
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;

    let dirs = HashMap::from([
        ('|', vec![(1, 0), (-1, 0)]),
        ('-', vec![(0, 1), (0, -1)]),
        ('L', vec![(-1, 0), (0, 1)]),
        ('J', vec![(-1, 0), (0, -1)]),
        ('7', vec![(1, 0), (0, -1)]),
        ('F', vec![(1, 0), (0, 1)]),
        ('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
    ]);
    let mut s = (-1, -1, 0);

    for i in 0..n {
        for j in 0..m {
            if mat[i as usize][j as usize] == 'S' {
                s.0 = i as i32;
                s.1 = j as i32;
            }
        }
    }

    let mut vis = vec![vec![false; m as usize]; n as usize];
    let mut queue = vec![s];
    let mut mx = 0;

    vis[s.0 as usize][s.1 as usize] = true;

    while !queue.is_empty() {
        let (ci, cj, cv) = queue.remove(0);
        mx = std::cmp::max(mx, cv);

        for (di, dj) in dirs.get(&mat[ci as usize][cj as usize]).unwrap() {
            if ci + di < 0 || ci + di >= n {
                continue;
            }
            if cj + dj < 0 || cj + dj >= m {
                continue;
            }
            if vis[(ci + di) as usize][(cj + dj) as usize] {
                continue;
            }
            if mat[(ci + di) as usize][(cj + dj) as usize] == '.' {
                continue;
            }
            if !check_dirs(
                (*di, *dj),
                dirs.get(&mat[(ci + di) as usize][(cj + dj) as usize])
                    .unwrap(),
            ) {
                continue;
            }

            queue.push((ci + di, cj + dj, cv + 1));
            vis[(ci + di) as usize][(cj + dj) as usize] = true;
        }
    }

    println!("Part 1 solution is: {}", mx);

    let mut start_dirs: Vec<(i32, i32)> = Vec::new();

    for (di, dj) in dirs.get(&'S').unwrap() {
        if s.0 + di < 0 || s.0 + di >= n {
            continue;
        }
        if s.1 + dj < 0 || s.1 + dj >= m {
            continue;
        }
        if mat[(s.0 + di) as usize][(s.1 + dj) as usize] == '.' {
            continue;
        }
        if !check_dirs(
            (*di, *dj),
            dirs.get(&mat[(s.0 + di) as usize][(s.1 + dj) as usize])
                .unwrap(),
        ) {
            continue;
        }

        start_dirs.push((*di, *dj));
    }

    for (key, dir) in &dirs {
        if *dir == start_dirs {
            mat[s.0 as usize][s.1 as usize] = *key;
        }
    }

    let mut br = 0;

    for i in 0..n {
        let mut is_in = false;
        let mut cor_dir = (0, 0);

        for j in 0..m {
            let ch = mat[i as usize][j as usize];
            if vis[i as usize][j as usize] {
                if ch == 'L' || ch == 'F' {
                    cor_dir = dirs.get(&ch).unwrap()[0];
                } else if ch == '7' || ch == 'J' {
                    if check_dirs(cor_dir, dirs.get(&ch).unwrap()) {
                        is_in = !is_in;
                    }
                } else if ch == '|' {
                    is_in = !is_in;
                }
            } else {
                br += if is_in { 1 } else { 0 };
            }
        }
    }

    println!("Part 2 solution is: {}", br);
}
