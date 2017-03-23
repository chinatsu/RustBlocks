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
    [10.0, 11.0, -1.0, 0.0],
    [10.0, 11.0, -1.0, 0.0]
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

pub const PIECES: [[[f64; 4]; 4]; 7] = [PIECE_I, PIECE_O, PIECE_T, PIECE_S, PIECE_Z, PIECE_J, PIECE_L];

pub struct Piece {
    pub id: u32,
    pub origin: u32,
    pub offset: [[f64; 4];4],
    pub orientation: u32,
    pub color: [f32; 4],
    pub rot: bool,
    pub soft_drop: bool,
    pub mov_left: bool,
    pub mov_right: bool,
    pub hard_drop: bool,
    pub surface_time: u64,
    pub drop_time: u64
}

impl Piece {
    pub fn new(id: u32, orientation: u32, offsets: [[f64; 4]; 4], color: [f32; 4]) -> Piece {
        Piece {
            id: id + 1,
            origin: SPAWN_POSITION,
            offset: offsets,
            orientation: orientation,
            color: color,
            rot: false,
            soft_drop: false,
            mov_left: false,
            mov_right: false,
            hard_drop: false,
            surface_time: 0,
            drop_time: 0
        }
    }


    pub fn draw(&mut self, id: i32, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in 0..self.offset[self.orientation as usize].len() {
                let x = (((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) % REAL_WIDTH as f64) - 1.0).floor() * CELL_SIZE as f64;
                let y = (20.0 - ((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) / REAL_WIDTH as f64).floor()) * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(get_color(id), s, c.transform, gl);
        }
    }

    pub fn lock(&mut self, m: &mut Matrix) {
        for i in 0..self.offset[self.orientation as usize].len() {
            m.add_piece(self.origin as i32 + self.offset[self.orientation as usize][i] as i32, self.id as i32);
        }
        m.clear_lines();
        self.new_piece();
    }

    pub fn rotate(&mut self, m: &Matrix, val: i32) {
        let old_orientation = self.orientation;
        let new_orientation = (self.orientation + val as u32) % self.offset[self.orientation as usize].len() as u32;
        if self.can_rotate(m, val) {
            self.orientation = new_orientation;
            return
        }
        let rotate_checks: [i32; 7] = [-1, 2, -12, -1, 2, 8, 4];
        for check in rotate_checks.iter() {
            self.origin = (self.origin as i32 + check) as u32;
            if self.can_rotate(m, val) {
                self.orientation = new_orientation;
                return
            }
        }
        self.origin -= 2;
    }

    pub fn move_down(&mut self, m: &mut Matrix) -> bool {
        if self.can_move(m, -11) {
            self.drop_time += 1;
            if self.drop_time % 10 == 0 {
                self.origin -= 11;
            }
            return false
        } else {
            self.surface_time += 1;
            if self.surface_time > 1250 {
                    self.lock(m);
                    self.new_piece();
                    self.surface_time = 0;
            }
            return true
        }
    }

    pub fn hard_drop(&mut self, m: &mut Matrix) {
        while self.can_move(m, -11) {
            self.origin -= 11;
        }
        self.lock(m);
        self.new_piece();
    }

    pub fn can_move(&mut self, m: &mut Matrix, val: i32) -> bool {
        for i in 0..self.offset[self.orientation as usize].len() {
            let index = self.origin as i32 + val + self.offset[self.orientation as usize][i as usize] as i32;
            if index <= 11 {
                return false
            }
            if index == ARRAY_SIZE as i32 {
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
        let choice = rand::thread_rng().gen_range(0, 7);
        self.offset = pcs[choice];
        self.id = choice as u32 + 1;
        self.origin = SPAWN_POSITION;
        self.orientation = 0;
    }
}

pub fn get_color(cell: i32) -> [f32; 4] {
    let c: [f32; 4];
    match cell {
        0 => {
            c = [0.0, 0.0, 0.0, 0.0];
        }
        1 => {
            c = [0.0, 1.0, 1.0, 1.0];
        }
        2 => {
            c = [1.0, 1.0, 0.0, 1.0];
        }
        3 => {
            c = [1.0, 0.0, 0.8, 1.0];
        }
        4 => {
            c = [0.0, 0.8, 0.0, 1.0];
        }
        5 => {
            c = [1.0, 0.0, 0.0, 1.0];
        }
        6 => {
            c = [0.0, 0.0, 1.0, 1.0];
        }
        7 => {
            c = [0.8, 0.5, 0.0, 1.0];
        }
        8 => {
            c = [1.0, 1.0, 1.0, 0.3];
        }
        _ => {
            c = [0.0, 0.0, 0.0, 0.0];
        }
    }
    c
}
