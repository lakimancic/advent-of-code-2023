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
        _ => println!("Invalid day!")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        call_solve(args[1].parse::<u32>().unwrap());
    }
    else {
        println!("Invalid number of arguments!");
    }
}
