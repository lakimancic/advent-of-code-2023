#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn new(line: &str) -> Self {
        let mut splits = line.split(" @ ");
        let mut xyz = splits.next().unwrap().split(", ");
        let mut vxyz = splits.next().unwrap().split(", ");

        Self {
            x: xyz.next().unwrap().trim().parse::<f64>().unwrap(),
            y: xyz.next().unwrap().trim().parse::<f64>().unwrap(),
            z: xyz.next().unwrap().trim().parse::<f64>().unwrap(),
            vx: vxyz.next().unwrap().trim().parse::<f64>().unwrap(),
            vy: vxyz.next().unwrap().trim().parse::<f64>().unwrap(),
            vz: vxyz.next().unwrap().trim().parse::<f64>().unwrap(),
        }
    }

    fn intersect(&self, line: &Self, min: f64, max: f64) -> bool {
        let d = self.vx * line.vy - self.vy * line.vx;
        if d == 0.0 {
            return false;
        }
        let b = (self.y * self.vx + line.x * self.vy - self.x * self.vy - line.y * self.vx) / d;
        let a = (line.x - self.x + b * line.vx) / self.vx;
        let nx = line.x + b * line.vx;
        let ny = line.y + b * line.vy;

        b > 0. && a > 0. && nx >= min && nx <= max && ny >= min && ny <= max
    }
}

pub fn solve() {
    const FILE_PATH: &str = "assets/input24.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let hstones = txt.lines().map(|x| Hailstone::new(x)).collect::<Vec<_>>();

    let mut cnt = 0usize;

    for i in 0..hstones.len() - 1 {
        for j in i + 1..hstones.len() {
            cnt += hstones[i].intersect(&hstones[j], 200000000000000., 400000000000000.) as usize;
        }
    }

    println!("Part 1 solution is: {}", cnt);
}
