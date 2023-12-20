use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Element {
    is_flip_flop: bool,
    state: bool,
    childs: Vec<String>,
    inputs: HashMap<String, bool>,
}

impl Element {
    fn parse(&mut self, input: bool, from: &str) -> bool {
        if self.is_flip_flop {
            if !input {
                self.state = !self.state;
                return true;
            }
        } else {
            self.inputs.insert(from.to_string(), input);
            let mut res = true;
            for (_, inp) in &self.inputs {
                res = res && *inp;
            }
            self.state = !res;
            return true;
        }
        false
    }

    fn new(is_ff: bool, st: bool, childs: Vec<String>) -> Self {
        Self {
            is_flip_flop: is_ff,
            state: st,
            childs: childs,
            inputs: HashMap::new(),
        }
    }

    fn add_input(&mut self, from: &str) {
        self.inputs.insert(from.to_string(), false);
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Self {
            is_flip_flop: self.is_flip_flop,
            state: self.state,
            childs: self.childs.clone(),
            inputs: self.inputs.clone(),
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut ma = a;
    let mut mb = b;

    while mb > 0 {
        let tmp = mb;
        mb = ma % mb;
        ma = tmp;
    }

    ma
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn button_click(
    graph: &mut HashMap<&str, Element>,
    broadcast: &Vec<String>,
    low_cnt: &mut usize,
    high_cnt: &mut usize,
    ind: &usize,
    prev: &mut HashMap<String, usize>,
    vis: &mut HashSet<String>,
    lcm_num: &mut usize,
    rx_parents: &Vec<String>,
) {
    let mut queue: Vec<String> = vec![];
    *low_cnt += 1 + broadcast.len();

    for child in broadcast {
        match graph.get_mut(child.as_str()) {
            Some(val) => {
                if val.parse(false, child.as_str()) {
                    queue.push(child.clone());
                }
            }
            None => (),
        }
    }

    while queue.len() > 0 {
        let node = queue.remove(0);
        let elem = graph.get(node.as_str()).unwrap();
        let new_input = elem.state;
        let childs = elem.childs.clone();

        if new_input {
            if rx_parents.contains(&node) {
                if !vis.contains(&node) && prev.contains_key(&node) {
                    *lcm_num = lcm(*lcm_num, ind - prev.get(&node).unwrap());
                    vis.insert(node.clone());
                }
                prev.insert(node.clone(), *ind);
            }
            *high_cnt += childs.len();
        } else {
            *low_cnt += childs.len();
        }

        for child in childs {
            match graph.get_mut(child.as_str()) {
                Some(val) => {
                    if val.parse(new_input, node.as_str()) {
                        queue.push(child.clone());
                    }
                }
                None => (),
            }
        }
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input20.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut graph: HashMap<&str, Element> = HashMap::new();
    let mut broadcast: Vec<String> = vec![];

    for line in txt.lines() {
        let mut splits = line.split(" -> ");
        let left = splits.next().unwrap();
        let right = splits.next().unwrap().split(", ");

        if left == "broadcaster" {
            broadcast = right.map(|x| x.to_string()).collect();
            continue;
        } else {
            let mut left_chars = left.chars();
            let first = left_chars.next().unwrap();
            let rest = left_chars.as_str();
            graph.insert(
                rest,
                Element::new(first == '%', false, right.map(|x| x.to_string()).collect()),
            );
        }
    }

    for line in txt.lines() {
        let mut splits = line.split(" -> ");
        let mut left = splits.next().unwrap();
        let right = splits.next().unwrap().split(", ");
        if left != "broadcaster" {
            let mut left_chars = left.chars();
            left_chars.next();
            left = left_chars.as_str();
        }

        for child in right {
            match graph.get_mut(child) {
                Some(value) => value.add_input(left),
                None => (),
            }
        }
    }

    let mut low_cnt = 0;
    let mut high_cnt = 0;
    let mut prev: HashMap<String, usize> = HashMap::new();
    let mut vis: HashSet<String> = HashSet::new();
    let mut lcm = 1usize;
    let rx_parents = graph
        .values()
        .find(|node| node.childs.contains(&"rx".to_string()))
        .unwrap()
        .inputs
        .keys()
        .map(|key| key.clone())
        .collect::<Vec<_>>();

    for i in 0..100000 {
        if i == 1000 {
            println!("Part 1 solution is: {}", low_cnt * high_cnt);
        }

        if vis.len() == rx_parents.len() {
            println!("Part 2 solution is: {}", lcm);
            break;
        }

        button_click(
            &mut graph,
            &broadcast,
            &mut low_cnt,
            &mut high_cnt,
            &i,
            &mut prev,
            &mut vis,
            &mut lcm,
            &rx_parents,
        );
    }
}
