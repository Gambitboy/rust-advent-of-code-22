use std::{fs::File, io::Read};

#[derive(Debug)]
struct RockPaperScissor {
    rock: bool,
    paper: bool,
    scissor: bool,
}

impl RockPaperScissor {
    fn rock() -> Self {
        Self {
            rock: true,
            paper: false,
            scissor: false,
        }
    }

    fn paper() -> Self {
        Self {
            rock: false,
            paper: true,
            scissor: false,
        }
    }

    fn scissor() -> Self {
        Self {
            rock: false,
            paper: false,
            scissor: true,
        }
    }

    fn eq(&self, other: &RockPaperScissor) -> bool {
        if (self.rock && other.rock)
            || (self.paper && other.paper)
            || (self.scissor && other.scissor)
        {
            return true;
        }

        false
    }

    fn check_win(&self, other: &RockPaperScissor) -> bool {
        if self.rock && other.paper {
            return false;
        }

        if self.paper && other.scissor {
            return false;
        }

        if self.scissor && other.rock {
            return false;
        }

        true
    }

    fn verse(&self, other: &RockPaperScissor) -> i32 {
        if self.eq(other) {
            return 3;
        }

        if self.check_win(other) {
            return 6;
        }

        return 0;
    }

    fn pick_score(&self) -> i32 {
        if self.rock {
            return 1;
        }

        if self.paper {
            return 2;
        }

        return 3;
    }
}

fn create_opponent(input: &str) -> RockPaperScissor {
    match input {
        "A" => RockPaperScissor::rock(),
        "B" => RockPaperScissor::paper(),
        "C" => RockPaperScissor::scissor(),
        _ => panic!("Invalid my input!"),
    }
}

fn create_me(input: &str) -> RockPaperScissor {
    match input {
        "X" => RockPaperScissor::rock(),
        "Y" => RockPaperScissor::paper(),
        "Z" => RockPaperScissor::scissor(),
        _ => panic!("Invalid my input!"),
    }
}

fn play_game(left: &str, right: &str) -> i32 {
    let opponent = create_opponent(left);
    let me = create_me(right);

    let game = me.verse(&opponent);

    game + me.pick_score()
}

fn main() {
    let mut file = File::open("input.txt").expect("Input should be there.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("There should be content in the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();
    println!("Lines in file: {}", lines.len());

    let mut score = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let theirs = split[0];
        let mine = split[1];

        let win = play_game(theirs, mine);
        score += win;
    }

    println!("Win: {}", score);
}
