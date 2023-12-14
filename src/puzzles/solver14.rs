use std::collections::HashMap;

fn move_north(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    for i in 0..*m {
        let mut c: usize = 0;
        for j in 0..*n {
            if mat[j][i] == 'O' {
                if c != j {
                    mat[c][i] = 'O';
                    mat[j][i] = '.';
                }
                c += 1;
            }
            else if mat[j][i] == '#' {
                c = j + 1;
            }
        }
    }
}

fn move_west(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    for i in 0..*n {
        let mut c: usize = 0;
        for j in 0..*m {
            if mat[i][j] == 'O' {
                if c != j {
                    mat[i][c] = 'O';
                    mat[i][j] = '.';
                }
                c += 1;
            }
            else if mat[i][j] == '#' {
                c = j + 1;
            }
        }
    }
}

fn move_south(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    for i in 0..*m {
        let mut c: usize = n-1;
        for j in (0..*n).rev() {
            if mat[j][i] == 'O' {
                if c != j {
                    mat[c][i] = 'O';
                    mat[j][i] = '.';
                }
                if c > 0 { c -= 1; }
            }
            else if mat[j][i] == '#' {
                if j > 0 { c = j - 1; }
            }
        }
    }
}

fn move_east(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    for i in 0..*n {
        let mut c: usize = m-1;
        for j in (0..*m).rev() {
            if mat[i][j] == 'O' {
                if c != j {
                    mat[i][c] = 'O';
                    mat[i][j] = '.';
                }
                if c > 0 { c -= 1; }
            }
            else if mat[i][j] == '#' {
                if j > 0 { c = j - 1; }
            }
        }
    }
}

fn cycle(mat: &mut Vec<Vec<char>>, n: &usize, m: &usize) {
    move_north(mat, n, m);
    move_west(mat, n, m);
    move_south(mat, n, m);
    move_east(mat, n, m);
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

    move_north(&mut mat, &n, &m);
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