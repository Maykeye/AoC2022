use crate::utils;
const AOC : u32 = 5;

#[derive(Debug, Clone)]
struct Movement
{
    from: usize,
    to: usize,
    count: usize
}
impl Movement {
    pub fn from_move_command(line: &str) -> Self{
        let mut tokens = line.split_whitespace();
        tokens.next(); // move
        let count = tokens.next().unwrap(); // 1
        tokens.next(); // from
        let from = tokens.next().unwrap(); // 2
        tokens.next(); // to
        let to = tokens.next().unwrap(); // 2
        Self {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            count: count.parse().unwrap()
        }
    }
}

struct Input {
    crates: Vec<Vec<char>>,
    moves: Vec<Movement>
}

impl Input {
    fn pop(&mut self, column:usize) -> char {
        for row in self.crates.iter_mut() {
            if row[column] != ' ' {
                let ch = row[column];
                row[column] = ' ';
                return ch;
            }
        }
        unreachable!()
    }

    fn push(&mut self, column:usize, crate_:char) {
        // add row if there is no empty spot
        if self.crates[0][column] != ' ' {
            let empty_row = vec![' '; self.crates[0].len()];
            self.crates.insert(0, empty_row);
        }

        for row in self.crates.iter_mut().rev() {
            if row[column] == ' ' {
                row[column] = crate_;
                return;
            }
        }
        unreachable!();
    }


    pub fn perform_movement(&mut self, mov_idx:usize){
        let mov = self.moves[mov_idx].clone();

        for _ in 0..mov.count {
            let ch = self.pop(mov.from - 1);
            self.push(mov.to - 1, ch);
            self.print();
        }
    }

    pub fn perform_multimovement(&mut self, mov_idx:usize){
        let mov = self.moves[mov_idx].clone();

        let buffer : Vec<char> = (0..mov.count)
            .map(|_| self.pop(mov.from - 1))
            .collect();

        for ch in buffer.iter().rev() {
            self.push(mov.to - 1, *ch);
        }
    }


    pub fn print(&self) {
        for row in &self.crates {
            let s : String = row.iter().collect();
            println!("{s}");
        }
        println!("XXXX");
    }

}

fn input() -> Input
{
    let lines = utils::input_file_lines(AOC);

    // STEP 1: parse crates
    let crate_indices = lines
        .iter()
        .position(|line| &line[1..2] == "1")
        .unwrap();
    let mut rows : Vec<Vec<char>> = lines[0..crate_indices]
        .iter()
        .map(|line| line[1..].chars().step_by(4).collect())
        .collect();
    let columns = rows.iter().map(|row| row.len()).max().unwrap();
    for row in rows.iter_mut() {
        for _ in 0..(columns - row.len()) {
            row.push(' ');
        }
    }

    //STEP2: movements
    let moves: Vec<Movement> = lines[crate_indices+2..]
        .iter()
        .map(|x| Movement::from_move_command(x))
        .collect()
    ;

    Input {
        crates: rows,
        moves
    }

}


fn part1() {
    let mut input = input();

    for i in 0..input.moves.len(){
        input.perform_movement(i);
    }

    let p1 : String = (0..input.crates[0].len())
        .map(|i| input.pop(i))
        .collect();

    dbg!(p1);
}

fn part2() {
    let mut input = input();

    for i in 0..input.moves.len(){
        input.perform_multimovement(i);
    }

    let p2 : String = (0..input.crates[0].len())
        .map(|i| input.pop(i))
        .collect();

    dbg!(p2);
}

pub fn run() {
    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }

}
