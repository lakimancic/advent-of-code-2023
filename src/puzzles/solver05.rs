use std::cmp;

#[derive(Debug)]
struct RangeSolver {
    ranges: Vec<(u64, u64, u64)>
}

impl RangeSolver {
    fn new(part: &str) -> RangeSolver {
        let mut parsed: Vec<(u64, u64, u64)> = Vec::new();
        let mut mini_parts = part.lines();
        mini_parts.next();

        for mini_part in mini_parts {
            let temp: Vec<u64> = mini_part.split_whitespace().filter_map(|x| x.trim().parse().ok()).collect();
            let temp = (temp[0], temp[1], temp[2]);
            parsed.push(temp);
        }

        RangeSolver { ranges: parsed }
    }

    fn solve_one(&self, n: u64) -> u64 {
        for (dest, source, size) in &self.ranges {
            if n >= *source && n < *source + *size {
                return n - source + dest;
            }
        }
        n
    }
}

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input05.txt";
    let txt = std::fs::read_to_string(FILE_PATH).expect("Error while reading file!");
    let mut parts = txt.split("\n\n").into_iter();

    let seeds: Vec<u64> = parts.next().unwrap().split_whitespace().filter_map(|x| x.trim().parse().ok()).collect();
    let mut range_solvers: Vec<RangeSolver> = Vec::new();

    for part in parts {
        range_solvers.push(RangeSolver::new(part));
    }

    let mut min_val = std::u64::MAX;
    for seed in &seeds {
        let mut temp_seed = *seed;
        for rsolver in &range_solvers {
            temp_seed = rsolver.solve_one(temp_seed);
        }
        min_val = cmp::min(min_val, temp_seed);
    }

    println!("Part 1 solution is: {}", min_val);
}