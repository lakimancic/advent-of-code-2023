use std::cmp::{min,max};

fn calc(mat: &Vec<Vec<char>>, k: usize) -> usize {
    let n = mat.len();
    let m = mat[0].len();
    let mut rows = vec![1; n];
    let mut cols = vec![1; m];

    for i in 0..n {
        let mut is_empty = true;
        for j in 0..m {
            if mat[i][j] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            rows[i] = k;
        }
    }

    for i in 0..m {
        let mut is_empty = true;
        for j in 0..n {
            if mat[j][i] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            cols[i] = k;
        }
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut sum: usize = 0;

    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            let si = min(galaxies[i].0, galaxies[j].0);
            let ei = max(galaxies[i].0, galaxies[j].0);
            let sj = min(galaxies[i].1, galaxies[j].1);
            let ej = max(galaxies[i].1, galaxies[j].1);

            for k in si..ei {
                sum += rows[k];
            }

            for k in sj..ej {
                sum += cols[k];
            }
        }
    }

    sum
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input11.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mat: Vec<Vec<char>> = txt.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

    println!("Part 1 solution is: {}", calc(&mat, 2));
    println!("Part 2 solution is: {}", calc(&mat, 1000000));
}