mod puzzles;

fn call_solve(day: u32) {
    println!("**** DAY {} ****", day);
    match day {
        1 => puzzles::solver01::solve(),
        2 => puzzles::solver02::solve(),
        3 => puzzles::solver03::solve(),
        4 => puzzles::solver04::solve(),
        5 => puzzles::solver05::solve(),
        6 => puzzles::solver06::solve(),
        7 => puzzles::solver07::solve(),
        8 => puzzles::solver08::solve(),
        9 => puzzles::solver09::solve(),
        10 => puzzles::solver10::solve(),
        11 => puzzles::solver11::solve(),
        12 => puzzles::solver12::solve(),
        13 => puzzles::solver13::solve(),
        14 => puzzles::solver14::solve(),
        15 => puzzles::solver15::solve(),
        16 => puzzles::solver16::solve(),
        17 => puzzles::solver17::solve(),
        18 => puzzles::solver18::solve(),
        19 => puzzles::solver19::solve(),
        20 => puzzles::solver20::solve(),
        21 => puzzles::solver21::solve(),
        22 => puzzles::solver22::solve(),
        23 => puzzles::solver23::solve(),
        24 => puzzles::solver24::solve(),
        25 => puzzles::solver25::solve(),
        _ => println!("Invalid day!"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        call_solve(args[1].parse::<u32>().unwrap());
    } else {
        for i in 1u32..26 {
            call_solve(i);
        }
    }
}
