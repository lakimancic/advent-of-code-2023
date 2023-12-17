use priority_queue::PriorityQueue;
use std::{collections::HashMap, cmp::Reverse};

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dijkstra(mat: &Vec<Vec<u32>>, minm: u32, maxm: u32) -> u32 {
    let mut pq: PriorityQueue<(u32, i32, i32, usize), Reverse<u32>> = PriorityQueue::new();
    let mut vis = vec![vec![[false;4];mat[0].len()];mat.len()];
    let mut dist: HashMap<(i32, i32, usize), u32> = HashMap::new();

    pq.push((0, 0, 0, usize::MAX), Reverse(0));

    while !pq.is_empty() {
        let ((d, ci, cj, dir), _) = pq.pop().unwrap();

        if ci as usize == mat.len() - 1 && cj as usize == mat[0].len() - 1 {
            return d;
        }
        
        if dir != usize::MAX {
            if vis[ci as usize][cj as usize][dir] {
                continue;
            }
            vis[ci as usize][cj as usize][dir] = true;
        }

        for ndir in 0..4 {
            let mut dinc: u32 = 0;
            if ndir == dir || (ndir + 2) % 4 == dir {
                continue;
            }
            for ndis in 1..maxm+1 {
                let ni = ci + DIRS[ndir].0 * (ndis as i32);
                let nj = cj + DIRS[ndir].1 * (ndis as i32);
                if ni < 0 || ni as usize >= mat.len() { continue; }
                if nj < 0 || nj as usize >= mat[0].len() { continue; }
                dinc += mat[ni as usize][nj as usize];
                if ndis < minm { continue; }
                let nd = d + dinc;
                let for_c = match dist.get(&(ni, nj, ndir)) {
                    Some(x) => *x,
                    None => u32::MAX
                };
                if for_c <= nd {
                    continue;
                }
                dist.insert((ni, nj, ndir), nd);
                pq.push((nd, ni, nj, ndir), Reverse(nd));
            }
        }
    }

    0
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input17.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mat = txt.lines().map(|line| line.chars().map(|ch| ch as u32 - 0x30).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    println!("Part 1 solution is: {}", dijkstra(&mat, 1, 3));
    println!("Part 2 solution is: {}", dijkstra(&mat, 4, 10));
}