extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use std::path::Path;
use rand::Rng;

use toml_config::ConfigFactory;
use tetris::*;
use graphics::*;
use config::*;

pub const PIECE_J: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64 -1.0), -1.0, 0.0, 1.0],
    [-(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), 0.0, REAL_WIDTH as f64],
    [-1.0, 0.0, 1.0, (REAL_WIDTH as f64 - 1.0)],
    [-(REAL_WIDTH as f64), 0.0, (REAL_WIDTH as f64), (REAL_WIDTH as f64 + 1.0)]
];
pub const PIECE_I: [[f64; 4]; 4] = [
    [-2.0, -1.0, 0.0, 1.0],
    [-(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64), 0.0, REAL_WIDTH as f64],
    [-(REAL_WIDTH as f64 + 2.0), -(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), -(REAL_WIDTH as f64 -1.0)],
    [(REAL_WIDTH as f64 - 1.0), -1.0, -(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64 * 2.0 + 1.0)]
];
pub const PIECE_Z: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64), -(REAL_WIDTH as f64 - 1.0), -1.0, 0.0],
    [-(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64), -(REAL_WIDTH as f64 - 1.0), 1.0],
    [-(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64 * 2.0 - 1.0), -(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64)],
    [-(REAL_WIDTH as f64 * 2.0 + 1.0), -(REAL_WIDTH as f64), -(REAL_WIDTH as f64 + 1.0), -0.0]
];
pub const PIECE_L: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64 + 1.0), -1.0, 0.0, 1.0],
    [-(REAL_WIDTH as f64), 0.0, (REAL_WIDTH as f64 - 1.0), (REAL_WIDTH as f64)],
    [-1.0, 0.0, 1.0, (REAL_WIDTH as f64 + 1.0)],
    [-(REAL_WIDTH as f64), -(REAL_WIDTH as f64 -1.0), 0.0, (REAL_WIDTH as f64)]
];
pub const PIECE_O: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), -1.0, 0.0],
    [-(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), -1.0, 0.0],
    [(REAL_WIDTH as f64 - 1.0), (REAL_WIDTH as f64), -1.0, 0.0],
    [(REAL_WIDTH as f64 - 1.0), (REAL_WIDTH as f64), -1.0, 0.0]
];
pub const PIECE_T: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64), -1.0, 0.0, 1.0],
    [-(REAL_WIDTH as f64), -1.0, 0.0, (REAL_WIDTH as f64)],
    [-1.0, 0.0, 1.0, (REAL_WIDTH as f64)],
    [-(REAL_WIDTH as f64), 0.0, 1.0, (REAL_WIDTH as f64)]
];
pub const PIECE_S: [[f64; 4]; 4] = [
    [-(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), 0.0, 1.0],
    [-(REAL_WIDTH as f64 * 2.0 - 1.0), -(REAL_WIDTH as f64), -(REAL_WIDTH as f64 -1.0), 0.0],
    [-(REAL_WIDTH as f64 * 2.0 + 1.0), -(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64 -1.0), -(REAL_WIDTH as f64)],
    [-(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64 + 1.0), -1.0, -(REAL_WIDTH as f64)]
];

pub const PENTA_I: [[f64; 5]; 4] = [
    [-2.0, -1.0, 0.0, 1.0, 2.0],
    [-(REAL_WIDTH as f64 * 2.0), -(REAL_WIDTH as f64), 0.0, REAL_WIDTH as f64, (REAL_WIDTH as f64 * 2.0)],
    [-(REAL_WIDTH as f64 + 2.0), -(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64), -(REAL_WIDTH as f64 - 1.0), -(REAL_WIDTH as f64 - 2.0)],
    [(REAL_WIDTH as f64 - 1.0), -1.0, -(REAL_WIDTH as f64 + 1.0), -(REAL_WIDTH as f64 * 2.0 + 1.0), -(REAL_WIDTH as f64 * 3.0 + 1.0)]
];


pub const PIECES: [[[f64; 4]; 4]; 7] = [PIECE_I, PIECE_O, PIECE_T, PIECE_S, PIECE_Z, PIECE_J, PIECE_L];

pub const PENTAS: [[[f64; 5]; 4]; 7] = [PENTA_I, PENTA_I, PENTA_I, PENTA_I, PENTA_I, PENTA_I, PENTA_I];

pub struct Piece {
    pub config: Config,
    pub id: u32,
    pub index: usize,
    pub origin: u32,
    pub offset: [[f64; 5];4],
    pub orientation: u32,
    pub color: [f32; 4],
    pub rot: bool,
    pub soft_drop: bool,
    pub mov_left: bool,
    pub mov_right: bool,
    pub hard_drop: bool,
    pub surface_time: u64,
    pub drop_time: u64,
    pub bag_index: usize,
    pub next_index: usize,
    pub bag: [usize; 7]
}

impl Piece {
    pub fn new(id: u32, offsets: [[f64; 5]; 4]) -> Piece {
        let mut result = Piece {
            config: ConfigFactory::load(Path::new("config.toml")),
            id: id + 1,
            index: id as usize,
            next_index: 0,
            origin: get_spawn_pos(),
            offset: offsets,
            orientation: 0,
            color: get_color(id as i32 + 1),
            rot: false,
            soft_drop: false,
            mov_left: false,
            mov_right: false,
            hard_drop: false,
            surface_time: 0,
            drop_time: 0,
            bag_index: 0,
            bag: [0, 1, 2, 3, 4, 5, 6]
        };
        let c = result.config.orientations;
        let or = [c.i, c.o, c.t, c.s, c.z, c.j, c.l];
        result.orientation = or[result.index];
        let length = result.bag.len();
        for _ in 0..length {
            let n = rand::thread_rng().gen_range(0, length);
            let tmp = result.bag[n];
            result.bag[n] = result.bag[length - 1];
            result.bag[length - 1] = tmp;
        }
        result
    }


    pub fn draw(&mut self, id: i32, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for i in 0..self.offset[self.orientation as usize].len() {
                let x = (((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) % REAL_WIDTH as f64) - 1.0).floor() * CELL_SIZE as f64;
                let y = (20.0 - ((self.origin as f64 + self.offset[self.orientation as usize][i as usize]) / REAL_WIDTH as f64).floor()) * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(get_color(id), s, c.transform, gl);
        }
    }
    pub fn draw_next(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let ref con = self.config;
        let next_piece = &PIECES[self.next_index][get_orientation(&con.orientations, self.next_index) as usize];
        for i in 0..next_piece.len() {
                let x = (((206.0 + next_piece[i as usize]) % REAL_WIDTH as f64) + 5.0).floor() * CELL_SIZE as f64;
                let y = (20.0 - ((206.0 + next_piece[i as usize]) / REAL_WIDTH as f64).floor()) * CELL_SIZE as f64;
                let s = rectangle::square(x, y, CELL_SIZE as f64);
                rectangle(get_color(self.next_index as i32 + 1), s, c.transform, gl);
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
        if self.can_move(m, -(REAL_WIDTH as i32)) {
            self.origin -= REAL_WIDTH;
            return false
        } else {
            self.surface_time += 1;
            if self.surface_time > 1250 {
                    self.lock(m);
                    self.surface_time = 0;
            }
            return true
        }
    }

    pub fn hard_drop(&mut self, m: &mut Matrix) {
        while self.can_move(m, -(REAL_WIDTH as i32)) {
            self.origin -= REAL_WIDTH;
        }
        self.lock(m);
    }

    pub fn can_move(&mut self, m: &mut Matrix, val: i32) -> bool {
        for i in 0..self.offset[self.orientation as usize].len() {
            let index = self.origin as i32 + val + self.offset[self.orientation as usize][i as usize] as i32;
            if index <= REAL_WIDTH as i32 {
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
        self.bag_index += 1;
        self.index = self.next_index;
        self.next_index = rand::thread_rng().gen_range(0, self.bag.len());
        let pcs = &PENTAS;
        let length = self.bag.len();
        if self.bag_index as usize >= self.bag.len() - 1 {
            for _ in 0..length {
                let n = rand::thread_rng().gen_range(0, length);
                let tmp = self.bag[n];
                self.bag[n] = self.bag[length - 1];
                self.bag[length - 1] = tmp;
            }
            self.next_index = self.bag[0];
            self.bag_index = 0;
        } else {
            self.next_index = self.bag[self.bag_index as usize + 1]
        }
        self.offset = pcs[self.index];
        self.id = self.index as u32 + 1;
        self.origin = get_spawn_pos();
        self.orientation = get_orientation(&self.config.orientations, self.index);
    }
}

pub fn get_spawn_pos() -> u32 {
    REAL_WIDTH * (REAL_HEIGHT - 1) - (((WIDTH as f64 + 1.0) / 2.0).floor() as u32)
}

pub fn get_orientation(or: &OrientationConfig, index: usize) -> u32 {
    let orientations: [u32; 7] = [or.i, or.o, or.t, or.s, or.z, or.j, or.l];
    orientations[index]
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
