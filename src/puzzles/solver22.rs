use std::cmp::*;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(str: &str) -> Self {
        let mut splits = str.split(",");
        let x = splits.next().unwrap().parse::<usize>().unwrap();
        let y = splits.next().unwrap().parse::<usize>().unwrap();
        let z = splits.next().unwrap().parse::<usize>().unwrap();

        Self { x, y, z }
    }
}

#[derive(Debug, Eq)]
struct Brick {
    from: Point,
    to: Point,
}

impl Brick {
    fn new(str: &str) -> Self {
        let mut splits = str.split("~");
        let from = Point::new(splits.next().unwrap());
        let to = Point::new(splits.next().unwrap());

        Self { from, to }
    }

    fn check_layer(&self, layer: &Vec<Vec<bool>>) -> bool {
        for y in self.from.y..self.to.y + 1 {
            for x in self.from.x..self.to.x + 1 {
                if layer[y][x] {
                    return true;
                }
            }
        }
        false
    }

    fn set_layer(&mut self, z_ind: usize, layers: &mut Vec<Vec<Vec<bool>>>) {
        for z in 0..self.to.z - self.from.z + 1 {
            for y in self.from.y..self.to.y + 1 {
                for x in self.from.x..self.to.x + 1 {
                    layers[z_ind + z][y][x] = true;
                }
            }
        }
        self.to.z = z_ind + self.to.z - self.from.z;
        self.from.z = z_ind;
    }

    fn rev_layer(&self, layer: &mut Vec<Vec<bool>>) {
        for y in self.from.y..self.to.y + 1 {
            for x in self.from.x..self.to.x + 1 {
                layer[y][x] = !layer[y][x];
            }
        }
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.z.cmp(&other.from.z)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.from.z == other.from.z
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input22.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut bricks = txt.lines().map(|line| Brick::new(line)).collect::<Vec<_>>();

    let mut max_p: Point = Point { x: 0, y: 0, z: 0 };

    for brick in &bricks {
        max_p.x = max(max_p.x, brick.to.x);
        max_p.y = max(max_p.y, brick.to.y);
        max_p.z = max(max_p.z, brick.to.z);
    }

    bricks.sort();

    let mut layers = vec![vec![vec![false; max_p.x + 1]; max_p.y + 1]; max_p.z + 1];

    for brick in &mut bricks {
        let mut z_ind = max_p.z;

        while z_ind > 0 && !brick.check_layer(&layers[z_ind - 1]) {
            z_ind -= 1;
        }

        brick.set_layer(z_ind, &mut layers);
    }

    bricks.sort();

    let mut cnt = 0usize;

    for i in 0..bricks.len() {
        let brick = &bricks[i];
        let mut j = i + 1;
        let mut can = true;

        brick.rev_layer(&mut layers[brick.to.z]);

        while j < bricks.len() && bricks[j].from.z <= bricks[i].to.z + 1 {
            if bricks[j].from.z < bricks[i].to.z + 1 {
                j += 1;
                continue;
            }
            can = can && bricks[j].check_layer(&layers[bricks[i].to.z]);
            j += 1;
        }

        brick.rev_layer(&mut layers[brick.to.z]);

        if can {
            cnt += 1;
        }
    }

    println!("Part 1 solution is: {}", cnt);
}
