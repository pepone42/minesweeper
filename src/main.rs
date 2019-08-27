mod towdarray;
mod minefield;
mod mine;

extern crate rand;
extern crate rustyline;

use minefield::{MineField, State};

fn parse_cmd(cmd: &str) -> Option<(usize,usize)> {
    let column_name="ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let v: Vec<&str> = cmd.split(',').collect();
    if v.len()!=2 {return None;}
    if let Ok(y) = v[0].trim().parse::<usize>() {
        if let Some(x) = column_name.find(v[1].trim()) {
            return Some((x,y));
        }
    }
    None
}

fn main() {
    let mut mf = MineField::new(16,12,10);
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        mf.show();
        if let Ok(input) = rl.readline(">> ") {
            match input.as_ref() {
                "quit" => break,
                cmd => {
                    if let Some((x,y)) = parse_cmd(cmd) {
                        match mf.try(x,y) {
                            State::Win => {
                                mf.show();
                                println!("You Win");
                                break;
                            },
                            State::GameOver => {
                                mf.show();
                                println!("You loose!");
                                break;
                            },
                            State::Continue => {},
                        }
                    };
                },
            }
        } else {
            break;
        }
    }
}
