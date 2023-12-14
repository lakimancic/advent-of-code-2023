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
    let n = mat.len();
    let m = mat[0].len();

    move_north(&mut mat, &n, &m);

    println!("Part 1 solution is: {}", calc_load(&mat, &n, &m));
}