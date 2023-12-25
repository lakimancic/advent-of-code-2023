use std::collections::{HashMap, HashSet, VecDeque};

fn key_to_usize(key: &str) -> usize {
    let mut sum = 0usize;
    for ch in key.bytes() {
        sum = sum * 26 + (ch as usize - 'A' as usize);
    }
    sum
}

fn insert_in_graph(graph: &mut HashMap<usize, Vec<usize>>, key: usize, value: usize) {
    match graph.get_mut(&key) {
        Some(edges) => {
            edges.push(value);
            ()
        }
        None => {
            graph.insert(key, vec![value]);
        }
    }
}

fn bfs(
    node: usize,
    graph: &HashMap<usize, Vec<usize>>,
    cnts: &mut HashMap<(usize, usize), usize>,
    vis: &mut HashSet<usize>,
    illegal: &Vec<(usize, usize)>,
) -> usize {
    if vis.contains(&node) {
        return 1;
    }
    let mut cnt = 0usize;
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(node);
    vis.insert(node);

    while !queue.is_empty() {
        let top = queue.pop_front().unwrap();
        cnt += 1;

        for new_node in graph.get(&top).unwrap() {
            if vis.contains(new_node) {
                continue;
            }
            let a = std::cmp::min(top, *new_node);
            let b = std::cmp::max(top, *new_node);

            if illegal.contains(&(a, b)) {
                continue;
            }

            match cnts.get(&(a, b)) {
                Some(val) => cnts.insert((a, b), *val + 1),
                None => cnts.insert((a, b), 1),
            };
            vis.insert(*new_node);
            queue.push_back(*new_node);
        }
    }

    cnt
}

fn find_bridges(graph: &mut HashMap<usize, Vec<usize>>) {
    let mut cnts: HashMap<(usize, usize), usize> = HashMap::new();
    let mut vis: HashSet<usize> = HashSet::new();
    let mut illegal = vec![];

    for node in graph.keys() {
        vis.clear();
        bfs(*node, graph, &mut cnts, &mut vis, &illegal);
    }

    let mut edges = cnts
        .iter()
        .map(|x| (x.1, x.0 .0, x.0 .1))
        .collect::<Vec<_>>();
    edges.sort();

    for _ in 0..3 {
        let edge = edges.pop().unwrap();
        illegal.push((edge.1, edge.2));
    }

    vis.clear();

    let mut res = 1usize;

    for node in graph.keys() {
        res *= bfs(*node, graph, &mut cnts, &mut vis, &illegal);
    }

    println!("Part 1 solution is: {}", res);
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input25.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in txt.lines() {
        let mut splits = line.split(": ");
        let key = key_to_usize(splits.next().unwrap());
        let values = splits
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| key_to_usize(val));

        for value in values {
            insert_in_graph(&mut graph, key, value);
            insert_in_graph(&mut graph, value, key);
        }
    }

    find_bridges(&mut graph);
}
