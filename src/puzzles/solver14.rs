use std::collections::HashMap;

fn move_dir(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize, dir1: bool, dir2: bool) {
    for i in 0..if dir2 { *m } else { *n } {
        let mut c: usize = if dir1 { 0 } else if dir2 { n-1 } else { m-1 };
        let range = if dir1 { 
            (0..if dir2 { *n } else { *m }).collect::<Vec<_>>() 
        } else { 
            (0..if dir2 { *n } else { *m }).rev().collect::<Vec<_>>() 
        };
        for j in range {
            if mat[if dir2 { j } else { i }][if dir2 { i } else { j }] == 'O' {
                if c != j {
                    mat[if dir2 { c } else { i }][if dir2 { i } else { c }] = 'O';
                    mat[if dir2 { j } else { i }][if dir2 { i } else { j }] = '.';
                }
                if dir1 {
                    c += 1;
                }
                else {
                    if c > 0 { c -= 1 };
                }
            }
            else if mat[if dir2 { j } else { i }][if dir2 { i } else { j }] == '#' {
                if dir1 {
                    c = j + 1;
                }
                else {
                    if j > 0 { c = j - 1 };
                }
            }
        }
    }
}

fn cycle(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    move_dir(mat, n, m, true, true);
    move_dir(mat, n, m, true, false);
    move_dir(mat, n, m, false, true);
    move_dir(mat, n, m, false, false);
}

fn calc_load(mat: &Vec<Vec<char>>, n: &usize, m: &usize) -> usize {
    let mut load: usize = 0;

    for i in 0..*n {
        let mut c: usize = 0;
        for j in 0..*m {
            if mat[i][j] == 'O' {
                c += 1;
            }
        }
        load += (n - i) * c;
    }

    load
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input14.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut mat = txt.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut mat2 = txt.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let n = mat.len();
    let m = mat[0].len();

    move_dir(&mut mat, &n, &m, true, true);
    println!("Part 1 solution is: {}", calc_load(&mat, &n, &m));

    let mut used: HashMap<String, usize> = HashMap::new();

    let mut ind: usize = 0;

    let mut key: String;

    loop {
        cycle(&mut mat2, &n, &m);

        key = mat2.iter().flat_map(|row| row.iter()).collect();

        if used.contains_key(&key) {
            break;
        }

        used.insert(key, ind);
        ind += 1;
    }

    let need_cycles = (1000000000 - used.get(&key).unwrap() - 1) % (ind - used.get(&key).unwrap());

    for _ in 0..need_cycles {
        cycle(&mut mat2, &n, &m);
    }

    println!("Part 2 solution is: {}", calc_load(&mat2, &n, &m));

}