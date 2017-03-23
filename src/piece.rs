extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use tetris::*;
use graphics::*;
use rand::Rng;

pub const PIECE_J: [[f64; 4]; 4] = [
    [-10.0, -1.0, 0.0, 1.0],
    [-12.0, -11.0, 0.0, 11.0],
    [-1.0, 0.0, 1.0, 10.0],
    [-11.0, 0.0, 11.0, 12.0]
];
pub const PIECE_I: [[f64; 4]; 4] = [
    [-2.0, -1.0, 0.0, 1.0],
    [-22.0, -11.0, 0.0, 11.0],
    [-13.0, -12.0, -11.0, -10.0],
    [10.0, -1.0, -12.0, -23.0]
];
pub const PIECE_Z: [[f64; 4]; 4] = [
    [-11.0, -10.0, -1.0, 0.0],
    [-22.0, -11.0, -10.0, 1.0],
    [-22.0, -21.0, -12.0, -11.0],
    [-23.0, -11.0, -12.0, -0.0]
];
pub const PIECE_L: [[f64; 4]; 4] = [
    [-12.0, -1.0, 0.0, 1.0],
    [-11.0, 0.0, 10.0, 11.0],
    [-1.0, 0.0, 1.0, 12.0],
    [-11.0, -10.0, 0.0, 11.0]
];
pub const PIECE_O: [[f64; 4]; 4] = [
    [-12.0, -11.0, -1.0, 0.0],
    [-12.0, -11.0, -1.0, 0.0],
    [-12.0, -11.0, -1.0, 0.0],
    [-12.0, -11.0, -1.0, 0.0]
];
pub const PIECE_T: [[f64; 4]; 4] = [
    [-11.0, -1.0, 0.0, 1.0],
    [-11.0, -1.0, 0.0, 11.0],
    [-1.0, 0.0, 1.0, 11.0],
    [-11.0, 0.0, 1.0, 11.0]
];
pub const PIECE_S: [[f64; 4]; 4] = [
    [-12.0, -11.0, 0.0, 1.0],
    [-21.0, -11.0, -10.0, 0.0],
    [-23.0, -22.0, -10.0, -11.0],
    [-22.0, -12.0, -1.0, -11.0]
];

pub const PIECES: [[[f64; 4]; 4]; 7] = [PIECE_T, PIECE_J, PIECE_Z, PIECE_O, PIECE_S, PIECE_L, PIECE_I];

pub struct Piece {
    pub origin: u32,
    pub offset: [[f64; 4];4],
    pub orientation: u32,
    pub color: [f32; 4],
    pub rot_l: bool,
    pub rot_r: bool,
    pub soft_drop: bool,
    pub mov_left: bool,
    pub mov_right: bool
}

impl Piece {
    pub fn new(orientation: u32, offsets: [[f64; 4]; 4], color: [f32; 4]) -> Piece {
        Piece {
            origin: SPAWN_POSITION,
            offset: offsets,
            orientation: orientation,
            color: color,
            rot_l: false,
            rot_r: false,
            soft_drop: false,
            mov_left: false,
            mov_right: false
        }
    }


    pub fn draw(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in 0..self.offset[self.orientation as usize].len() {
            //if piece.origin as f64 + piece.offset[piece.orientation as usize][i as usize] < 231.0 {
                let x = ((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) % REAL_WIDTH as f64).floor() * CELL_SIZE as f64;
                let y = (21.0 - ((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) / REAL_WIDTH as f64).floor()) * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(self.color, s, c.transform, gl);
            //}
        }
    }

    pub fn lock(&mut self, m: &mut Matrix) {
        for i in 0..self.offset[self.orientation as usize].len() {
            m.add_piece(self.origin as i32 + self.offset[self.orientation as usize][i] as i32, 2);
        }
        m.clear_lines();
        self.new_piece();
    }
    pub fn rotate(&mut self, cw: bool) {
        if cw {
            self.orientation = (self.orientation + 1) % self.offset[self.orientation as usize].len() as u32
        } else {
            self.orientation = (self.orientation + 3) % self.offset[self.orientation as usize].len() as u32
        }
    }
    pub fn move_down(&mut self, m: &mut Matrix) {
        if self.can_move(m, -11) {
            self.origin -= 11;
        } else {
            self.lock(m);
            self.new_piece();
        }
    }

    pub fn can_move(&mut self, m: &mut Matrix, val: i32) -> bool {
        for i in 0..self.offset[self.orientation as usize].len() {
            let index = self.origin as i32 + val + self.offset[self.orientation as usize][i as usize] as i32;
            if index <= 11 {
                return false
            }
            if m.state[index as usize] != 0 {
                return false
            }
        }
        true
    }

    pub fn can_rotate(&mut self, m: &Matrix, val: i32) -> bool {
        let new_orientation = (self.orientation as i32 + val) % self.offset.len() as i32;
        for i in 0..self.offset[self.orientation as usize].len() {
            let index = self.origin as i32 + self.offset[new_orientation as usize][i as usize] as i32;
            if m.state[index as usize] != 0 {
                return false
            }
        }
        true
    }

    pub fn new_piece(&mut self) {
        let pcs = &PIECES;
        let choice = rand::thread_rng().choose(pcs).unwrap();
        self.offset = *choice;
        self.origin = SPAWN_POSITION;
        self.orientation = 0;
    }
}
