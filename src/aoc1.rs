use crate::utils;

type ElfKnapsack = Vec<u32>;

const AOC: u32 = 1;

struct AOC1 {
    pub elves: Vec<ElfKnapsack>,
}

enum AOC1InputLine {
    EndOfKnapSnack,
    Snack(u32),
}

impl From<&str> for AOC1InputLine {
    fn from(line: &str) -> Self {
        if line.is_empty() {
            AOC1InputLine::EndOfKnapSnack
        } else {
            AOC1InputLine::Snack(line.parse().unwrap())
        }
    }
}

fn input_file() -> AOC1 {
    let input = utils::input_file_lines(AOC);

    let mut elves = vec![vec![0]];

    for line in input.iter().map(|x| AOC1InputLine::from(x.as_ref())) {
        match line {
            AOC1InputLine::Snack(calories) => {
                let last = elves.last_mut().unwrap();
                last.push(calories);
            }
            AOC1InputLine::EndOfKnapSnack => {
                elves.push(vec![]);
            }
        }
    }

    AOC1 { elves }
}

fn part1(elves: AOC1){
    let elves = elves.elves;

    let fattest= elves.iter()
        .map(|knapsack| knapsack.iter().sum::<u32>())
        .max()
    ;
    println!("{}", fattest.unwrap())
}

fn part2(elves: AOC1){
    let elves = elves.elves;

    let mut summed : Vec<u32> = elves.iter()
        .map(|knapsack| knapsack.iter().sum::<u32>())
        .collect()
    ;
    summed.sort();
    let top3 : u32 = summed.as_slice()[summed.len() - 3..].iter().sum();
    println!("{}", top3);
}

pub fn run() {
    println!("Hello, world!");
    let elves = input_file();
    if utils::is_part_1() {
        part1(elves);
    } else {
        part2(elves)
    }

}
