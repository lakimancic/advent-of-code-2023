use std::collections::HashMap;

fn solve_dp(dp: &mut HashMap<(usize, usize, usize), usize>, types: &Vec<char>, blocks: &Vec<usize>, di: usize, bi: usize, ci: usize) -> usize {
    let key = (di, bi, ci);
    if dp.contains_key(&key) {
        return *dp.get(&key).unwrap();
    }

    if di == types.len() {
        if bi == blocks.len() && ci == 0 {
            return 1;
        }
        else if bi == blocks.len()-1 && blocks[bi] == ci {
            return 1;
        }
        else {
            return 0;
        }
    }

    let mut sum: usize = 0;

    for ch in ['.', '#'] {
        if types[di] == ch || types[di] == '?' {
            if ch == '.' && ci == 0 {
                sum += solve_dp(dp, types, blocks, di+1, bi, 0);
            }
            else if ch == '.' && ci > 0 && bi < blocks.len() && blocks[bi] == ci {
                sum += solve_dp(dp, types, blocks, di+1, bi+1, 0);
            }
            else if ch == '#' {
                sum += solve_dp(dp, types, blocks, di+1, bi, ci+1);
            }
        }
    }
    dp.insert(key, sum);

    sum
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input12.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut sum: usize = 0;
    let mut sum2: usize = 0;
    
    for line in txt.lines() {
        let mut dp: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let splits = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let types = splits[0].chars().collect::<Vec<char>>();
        let blocks = splits[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        sum += solve_dp(&mut dp, &types, &blocks, 0, 0, 0);

        dp.clear();
        let types = vec![splits[0];5].join("?").chars().collect::<Vec<char>>();
        let blocks = vec![splits[1];5].join(",").split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        sum2 += solve_dp(&mut dp, &types, &blocks, 0, 0, 0);
    }

    println!("Part 1 solution is: {}", sum);
    println!("Part 2 solution is: {}", sum2);
}