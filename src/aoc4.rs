use crate::utils;
const AOC : u32 = 4;
use std::ops::RangeInclusive;


#[derive(Debug)]
struct Pair {
    pub first : RangeInclusive<u32>,
    pub second : RangeInclusive<u32>
}

type Pairs = Vec<Pair>;


impl Pair {

    fn decode_range(line : &str) -> RangeInclusive<u32> {
        let hyphen_pos = line.find("-").unwrap();
        let (lhs, rhs) = line.split_at(hyphen_pos);
        let rhs = &rhs[1..]; //SKIP hyphen
        let lhs = lhs.parse().unwrap();
        let rhs = rhs.parse().unwrap();
        lhs..=rhs
    }

    fn new(pair_line: &str) -> Self {
        let comma_pos = pair_line.find(",").unwrap();
        let (lhs, rhs) = pair_line.split_at(comma_pos);
        let rhs = &rhs[1..]; //SKIP comma
        Self {
            first: Self::decode_range(lhs),
            second: Self::decode_range(rhs),
        }


    }
}

fn input() -> Pairs {
    utils::input_file_lines(AOC)
    .iter()
    .map(|x| Pair::new(x))
    .collect()
}

fn is_range_in_range(
    needle : &RangeInclusive<u32>,
    haystack : &RangeInclusive<u32>) -> bool
{
    haystack.contains(needle.start()) &&
    haystack.contains(needle.end())
}

fn is_range_overlaps(a: &RangeInclusive<u32>, b:&RangeInclusive<u32>) -> bool
{
    // Behold
    // [------]
    //    [-------]
    //      [---]

    let start = a.start().max(b.start());
    let end   = a.end().min(b.end());

    start <= end
}


fn part1() {
    let input = input();
    let p1 = input
        .iter()
        .filter(|pair|
               is_range_in_range(&pair.first, &pair.second)
            || is_range_in_range(&pair.second, &pair.first)
        )
        .count();
    dbg!(p1);
}

fn part2() {
    let input = input();

    let p2 = input
    .iter()
    .filter(|pair| is_range_overlaps(&pair.first, &pair.second))
    .count()
    ;






    dbg!(p2);
}


pub fn run() {

    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }


}
