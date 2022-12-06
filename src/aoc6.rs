
use crate::utils;
const AOC : u32 = 6;

fn input() -> Vec<char> {
    let lines = utils::input_file_lines(AOC);
    let line = lines.first().unwrap();
    line.chars().collect()
}

fn part1() {
    let input = input();

    let p1 = (3..input.len())
    .find(|&pos|{
        let a = input[pos-3];
        let b = input[pos-2];
        let c = input[pos-1];
        let d = input[pos];

        let a_ok = a != b && a != c && a != d;
        let b_ok =           b != c && b != d;
        let c_ok =                     c != d;

        a_ok && b_ok && c_ok
    }).unwrap() + 1;

    dbg!(p1 );
}


fn part2() {
    let input = input();

    let p2 = (13..input.len())
    .find(|&pos|{
        let chunk = &input[pos-13..=pos];
        for (i, ch) in chunk.iter().enumerate(){
            let other = chunk[i+1..].iter().find(|&c| c == ch);
            if other.is_some(){
                return false
            }
        }
        true
    }).unwrap() + 1;

    dbg!(p2);
}



pub fn run() {
    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }

}
