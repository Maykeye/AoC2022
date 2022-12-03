use crate::utils;

const AOC: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissor,
}

impl Action {
    pub fn from_abc(ch: char) -> Self {
        match ch {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissor,
            _ => unimplemented!(),
        }
    }

    pub fn from_xyz(ch: char) -> Self {
        match ch {
            'X' => Action::Rock,
            'Y' => Action::Paper,
            'Z' => Action::Scissor,
            _ => todo!(),
        }
    }

    pub fn wins_to(self) -> Self {
        match self {
            Action::Rock => Action::Scissor,
            Action::Paper => Action::Rock,
            Action::Scissor => Action::Paper,
        }
    }
    pub fn draws_to(self) -> Self {
        self
    }

    pub fn loses_to(self) -> Self {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissor,
            Action::Scissor => Action::Rock,
        }
    }

    pub fn round_points(opponent: Action, me: Action) -> u32 {
        if opponent == me {
            3 + me.value()
        } else if opponent.loses_to() == me {
            6 + me.value()
        } else {
            me.value()
        }
    }

    pub fn value(self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissor => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Action,
    me: char,
}

type AOC2 = Vec<Round>;

fn input() -> AOC2 {
    utils::input_file_lines(AOC)
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let lhs = split.next().unwrap();
            let lhs = Action::from_abc(lhs.chars().next().unwrap());
            let rhs = split.next().unwrap();
            let rhs = rhs.chars().next().unwrap();
            Round {
                opponent: lhs,
                me: rhs,
            }
        })
        .collect()
}

fn part1() {
    let input = input();
    let total_score: u32 = input
        .iter()
        .map(|round| {
            let me = Action::from_xyz(round.me);
            let round_score = Action::round_points(round.opponent, me);
            round_score
        })
        .sum();
    println!("{}", total_score);
}

fn part2() {
    let input = input();
    let total_score: u32 = input
        .iter()
        .map(|round| {
            let me = match round.me {
                'X' => round.opponent.wins_to(),
                'Y' => round.opponent.draws_to(),
                'Z' => round.opponent.loses_to(),
                _ => unimplemented!("not expected"),
            };
            let round_score = Action::round_points(round.opponent, me);
            round_score
        })
        .sum();
    println!("{}", total_score);
}

pub fn run() {
    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }

    println!("Hello, world!");
}
