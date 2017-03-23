extern crate graphics;
extern crate opengl_graphics;
use graphics::*;
use piece::*;


pub const WIDTH: u32 = 10;
pub const HEIGHT: u32 = 20;
pub const HIDDEN: u32 = 5;
pub const REAL_WIDTH: u32 = WIDTH + 1;
pub const REAL_HEIGHT: u32 = HEIGHT + HIDDEN + 2;

pub const ARRAY_SIZE: u32 = REAL_WIDTH * REAL_HEIGHT;
pub const TOP_ROW: u32 = ARRAY_SIZE - REAL_WIDTH;

pub const CENTER_COLUMN: u32 = 5;
pub const CENTER_TOP_ROW: u32 = TOP_ROW - CENTER_COLUMN;

pub const SPAWN_POSITION: u32 = 281;

pub const CELL_SIZE: u32 = 32;

pub struct Matrix {
    pub state: [i32; ARRAY_SIZE as usize],
    pub lines_cleared: u64,
}

impl Matrix {
    pub fn new() -> Matrix {
        let mut m = Matrix {
            state: [0i32; ARRAY_SIZE as usize],
            lines_cleared: 0
        };
        m.clear_matrix();
        m
    }

    pub fn add_piece(&mut self, i: i32, id: i32) {
        self.state[i as usize] = id;
        let l = i - (i % REAL_WIDTH as i32);
        self.state[l as usize] -= 1;
    }

    pub fn clear_lines(&mut self) {
        let mut i = TOP_ROW as usize;
        while i > 0 {
            if self.state[i] == -11 {
                self.lines_cleared += 1;
                if self.lines_cleared % 10 == 0 {
                    println!("{} lines cleared", self.lines_cleared);
                }
                for j in i..TOP_ROW as usize {
                    self.state[j] = self.state[j + 11];
                }
                if self.state[TOP_ROW as usize] != -1 {
                    self.state[TOP_ROW as usize] = -1;
                    for k in TOP_ROW as usize+1..ARRAY_SIZE as usize {
                        self.state[k] = 0;
                    }
                }
            }
            i -= 1;
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in 0..231 as u32 {
            if self.state[i as usize] > 0 {
                let x = ((i as f64 % REAL_WIDTH as f64).floor() - 1.0) * CELL_SIZE as f64;
                let y = (21.0 - (i as f64 / REAL_WIDTH as f64)).floor() * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(get_color(self.state[i as usize]), s, c.transform, gl);

            }
        }
    }

    fn clear_matrix(&mut self) {
        for i in 0..ARRAY_SIZE {
            if i < REAL_WIDTH || i > ARRAY_SIZE || i % REAL_WIDTH == 0 {
                self.state[i as usize] = -1;
            } else {
                self.state[i as usize] = 0;
            }
        }
    }

}
