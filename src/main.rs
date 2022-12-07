mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;

mod utils;

fn force_aoc(n: i32) -> bool {
    let key = format!("AOC{n}");
    std::env::var(key).is_ok()
}

fn run_forced() -> bool {
    let aocs = [
        aoc1::run,
        aoc2::run,
        aoc3::run,
        aoc4::run,
        aoc5::run,
        aoc6::run,
        aoc7::run,
    ];

    for (i, func) in aocs.iter().enumerate() {
        if force_aoc(i as i32 + 1) {
            func();
            return true;
        }
    }
    false
}


fn main() {
    if ! run_forced() {
        aoc7::run();
    }


}
