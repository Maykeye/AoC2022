
use std::fs::File;
use std::io::{self, BufRead};

pub fn input_file_path(aoc : u32) -> String {
    if std::env::var("SAMPLE").is_ok() {
        return format!("sample/aoc{}.txt", aoc);
    }
    return format!("input/aoc{}.txt", aoc);

}

pub fn is_part_1() -> bool {
    ! std::env::var("P2").is_ok()
}

pub fn input_file_lines(aoc: u32) -> Vec<String>
{
    let filename = input_file_path(aoc);
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().flatten().collect()
}
