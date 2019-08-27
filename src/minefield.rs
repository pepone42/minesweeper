

use rand::prelude::*;
// use std::{thread, time};

use towdarray::TowDArray;
use mine::Mine;

#[derive(Debug)]
pub struct MineField {
    grid: TowDArray<Mine>,
    bomb_count: u8,
    discovered_cell: usize,
}
#[derive(Debug)]
pub enum State {
    GameOver,
    Win,
    Continue,
}

const NEIGHBOR_POSITION: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0),
                                                (-1, 1), (0, 1), (1, 1)];

impl MineField {
    pub fn new(w: u8, h: u8, bomb_count: u8) -> Self {
        let mut m = MineField {
            grid: TowDArray::<Mine>::new(w as usize, h as usize),
            bomb_count: bomb_count,
            discovered_cell: 0,
        };
        for i in 0..bomb_count {
            let p = m.grid.position_to_point(i as usize);
            m.grid[p].is_bomb = true;
        }
        //thread_rng().shuffle(&mut m.grid);
        let mut rng = rand::thread_rng();
        m.grid.shuffle(&mut rng);
        m.calculate_neighbor_count();
        m
    }
    fn is_relative_position_valid(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let x = x as isize;
        let y = y as isize;
        let w = self.grid.w as isize;
        let h = self.grid.h as isize;
        !(dx + x < 0 || dy + y < 0 || x + dx >= w || y + dy >= h)
    }
    fn get_relative_cell_mut(&mut self,
                             x: usize,
                             y: usize,
                             dx: isize,
                             dy: isize)
                             -> Option<&mut Mine> {

        if self.is_relative_position_valid(x, y, dx, dy) {
            // We limit our grid and input to u8xu8
            // so this should not be an issue
            let x = x as isize;
            let y = y as isize;
            let x = (x + dx) as usize;
            let y = (y + dy) as usize;
            Some(&mut self.grid[(x, y)])
        } else {
            None
        }
    }
    fn calculate_neighbor_count(&mut self) {
        let bomb_coord = self.grid
            .iter()
            .enumerate()
            .filter(|&(_, m)| m.is_bomb)
            .map(|(i, _)| self.grid.position_to_point(i))
            .collect::<Vec<_>>();
        for b in bomb_coord {
            for r in &NEIGHBOR_POSITION {
                let (x, y) = b;
                let (dx, dy) = *r;
                if let Some(mine) = self.get_relative_cell_mut(x, y, dx, dy) {
                    mine.neighbor_count += 1;
                }
            }
        }
    }
    pub fn show(&self) {
        print!("{}[2J", 27 as char);
        print!("   ");
        for column in "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .take(self.grid.w) {
            print!(" {}", column);
        }

        for (index, mine) in self.grid.iter().enumerate() {
            let (x, y) = self.grid.position_to_point(index);
            if x == 0 {
                println!("");
                print!("{:03} ", y);
            }
            match *mine {
                Mine { is_visible: false, .. } => print!("▩ "),
                Mine { neighbor_count: 0, is_bomb: false, .. } => print!("  "),
                Mine { neighbor_count: count, is_bomb: false, .. } => print!("{} ", count),
                Mine { is_bomb: true, .. } => print!("x "),
            }
        }
        println!();
    }
    fn discover_all_bomb(&mut self) {
        for bomb in self.grid
            .iter()
            .enumerate()
            .filter(|&(_, m)| m.is_bomb)
            .map(|(i, _)| self.grid.position_to_point(i))
            .collect::<Vec<_>>() {
                self.discover(bomb.0,bomb.1);
            }

    }
    pub fn is_inupt_valid(&self, x: usize, y: usize) -> bool {
        !(x > u8::max_value() as usize || y > u8::max_value() as usize || x >= self.grid.w ||
          y >= self.grid.h)
    }
    pub fn discover(&mut self, x: usize, y: usize) {
        if !self.is_inupt_valid(x, y) || self.grid[(x, y)].is_visible {
            return;
        }
        self.grid[(x, y)].is_visible = true;
        self.discovered_cell += 1;
        if self.grid[(x, y)].neighbor_count == 0 && !self.grid[(x, y)].is_bomb {
            for r in &NEIGHBOR_POSITION {
                let (dx, dy) = *r;
                if self.is_relative_position_valid(x, y, dx, dy) {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    self.discover(nx as usize, ny as usize);
                }
            }
        }
    }
    pub fn try(&mut self, x: usize, y: usize) -> State {
        if self.is_inupt_valid(x, y) {
            self.discover(x, y);
            if self.grid[(x, y)].is_bomb {
                self.discover_all_bomb();
                return State::GameOver;
            }
            if self.bomb_count as usize + self.discovered_cell == self.grid.w * self.grid.h {
                self.discover_all_bomb();
                return State::Win;
            }
        }
        State::Continue
    }
}
