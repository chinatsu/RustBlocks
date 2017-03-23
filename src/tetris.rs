extern crate graphics;
extern crate opengl_graphics;
use graphics::*;


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
    pub state: [i32; ARRAY_SIZE as usize]
}

impl Matrix {
    pub fn new() -> Matrix {
        let mut m = Matrix {
            state: [0i32; ARRAY_SIZE as usize]
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
        for i in 0..TOP_ROW as usize {
            if self.state[i] == -11 {
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
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in 0..231 as u32 {
            if self.state[i as usize] > 0 {
                let x = (i as f64 % REAL_WIDTH as f64).floor() * CELL_SIZE as f64;
                let y = (21.0 - (i as f64 / REAL_WIDTH as f64)).floor() * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(self.get_color(i as usize), s, c.transform, gl);

            }
        }
    }

    fn get_color(&mut self, cell: usize) -> [f32; 4] {
        let c: [f32; 4];
        match self.state[cell] {
            0 => {
                c = [0.0, 0.0, 0.0, 0.0];
            }
            1 => {
                c = [1.0, 1.0, 1.0, 1.0];
            }
            2 => {
                c = [0.0, 0.0, 1.0, 1.0];
            }
            3 => {
                c = [1.0, 0.0, 0.0, 1.0];
            }
            4 => {
                c = [1.0, 1.0, 0.0, 1.0];
            }
            5 => {
                c = [0.0, 1.0, 0.0, 1.0];
            }
            6 => {
                c = [1.0, 0.0, 1.0, 1.0];
            }
            7 => {
                c = [0.0, 1.0, 1.0, 1.0];
            }
            _ => {
                c = [0.0, 0.0, 0.0, 0.0];
            }
        }
        c
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
