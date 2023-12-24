use z3::ast::Ast;

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

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x = z3::ast::Int::new_const(&ctx, "x");
    let y = z3::ast::Int::new_const(&ctx, "y");
    let z = z3::ast::Int::new_const(&ctx, "z");
    let vx = z3::ast::Int::new_const(&ctx, "vx");
    let vy = z3::ast::Int::new_const(&ctx, "vy");
    let vz = z3::ast::Int::new_const(&ctx, "vz");

    for (i, hstone) in hstones.iter().enumerate() {
        let cx = z3::ast::Int::from_i64(&ctx, hstone.x as i64);
        let cvx = z3::ast::Int::from_i64(&ctx, hstone.vx as i64);
        let cy = z3::ast::Int::from_i64(&ctx, hstone.y as i64);
        let cvy = z3::ast::Int::from_i64(&ctx, hstone.vy as i64);
        let cz = z3::ast::Int::from_i64(&ctx, hstone.z as i64);
        let cvz = z3::ast::Int::from_i64(&ctx, hstone.vz as i64);

        let t = z3::ast::Int::new_const(&ctx, format!("T{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&ctx, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(cx + cvx * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(cy + cvy * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(cz + cvz * t.clone())));
    }

    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            return;
        };
        println!(
            "Part 2 solution is: {}",
            m.eval(&(x + y + z), true).unwrap()
        );
    }
}
