use crate::utils;
const AOC : u32 = 3;

#[derive(Debug)]
struct Backpack {
    pub left : Vec<char>,
    pub right : Vec<char>
}


impl Backpack {
    fn new(items: &str) -> Self {
        let (lhs, rhs) = items.split_at(items.len()/ 2);
        Self {
            left : lhs.chars().collect(),
            right: rhs.chars().collect(),
        }
    }

    fn has_item(&self, ch : char) -> bool {
        self.left.iter().chain(&self.right) .find(|backpack_item |{
            **backpack_item == ch
        }).is_some()
    }


}

fn item_priority(item : char) -> u32 {
    if 'a' <= item && item <= 'z' {
        (item as u32) - ('a' as u32) + 1
    } else {
        (item as u32) - ('A' as u32) + 27
    }
}


fn input() -> Vec<Backpack>
{
    utils::input_file_lines(AOC)
        .iter()
        .map(|line| Backpack::new(&line))
        .collect()
}

fn part1() {
    let backpacks = input();

    let priorities : u32 = backpacks.iter().map(|backpack|{
        let mut max_priority = 0;

        for item_in_left in backpack.left.iter()
        {
            let right = backpack.right
                .iter()
                .find(|item_in_right| **item_in_right == *item_in_left);
            if right.is_none() {
                continue;
            }
            max_priority = max_priority.max(item_priority(*item_in_left));
        }
        max_priority
    }).sum();
    dbg!(priorities);
}

fn part2() {
    let backpacks = input();

    let badges : u32 = backpacks.chunks_exact(3).map(|group|{
        let g1 = &group[0];
        let g2 = &group[1];
        let g3 = &group[2];

        let badge = g1.left
            .iter()
            .chain(g1.right.iter())
            .find(|&ch| {
            g2.has_item(*ch) && g3.has_item(*ch)
        }).unwrap();

        item_priority(*badge)
    }).sum();
    dbg!(badges);


}



pub fn run() {
    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }


}
