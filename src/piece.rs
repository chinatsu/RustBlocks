use tetris::*;

pub const PIECE_J: [[f64; 4]; 4] = [[-10.0, -1.0, 0.0, 1.0], [-12.0, -11.0, 0.0, 11.0], [-1.0, 0.0, 1.0, 10.0], [-11.0, 0.0, 11.0, 12.0]];
pub const PIECE_I: [[f64; 4]; 4] = [[-2.0, -1.0, 0.0, 1.0], [-22.0, -11.0, 0.0, 11.0], [-13.0, -12.0, -11.0, -10.0], [10.0, -1.0, -12.0, -23.0]];
pub const PIECE_Z: [[f64; 4]; 4] = [[-11.0, -10.0, -1.0, 0.0], [-22.0, -11.0, -10.0, 1.0], [-22.0, -21.0, -12.0, -11.0], [-23.0, -11.0, -12.0, -0.0]];
pub const PIECE_L: [[f64; 4]; 4] = [[-12.0, -1.0, 0.0, 1.0], [-11.0, 0.0, 10.0, 11.0], [-1.0, 0.0, 1.0, 12.0], [-11.0, -10.0, 0.0, 11.0]];
pub const PIECE_O: [[f64; 4]; 4] = [[-1.0, -1.0, 0.0, 1.0], [-12.0, -1.0, 0.0, 1.0], [-12.0, -1.0, 0.0, 1.0], [-1.0, -1.0, 0.0, 1.0]];
pub const PIECE_T: [[f64; 4]; 4] = [[-11.0, -1.0, 0.0, 1.0], [-11.0, -1.0, 0.0, 11.0], [-1.0, 0.0, 1.0, 11.0], [-11.0, 0.0, 1.0, 11.0]];
pub const PIECE_S: [[f64; 4]; 4] = [[-12.0, -11.0, 0.0, 1.0], [-21.0, -11.0, -10.0, 0.0], [-23.0, -22.0, -10.0, -11.0], [-22.0, -12.0, -1.0, -11.0]];

pub const PIECES: [[[f64; 4]; 4]; 7] = [PIECE_T, PIECE_J, PIECE_Z, PIECE_O, PIECE_S, PIECE_L, PIECE_I];

pub struct Piece {
    pub size: u32,
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
    pub fn new(orientation: u32, size: u32, offsets: [[f64; 4]; 4], color: [f32; 4]) -> Piece {
        Piece {
            size: size,
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
    pub fn rotate(&mut self, cw: bool) {
        if cw {
            self.orientation = (self.orientation + 1) % self.offset[self.orientation as usize].len() as u32
        } else {
            self.orientation = (self.orientation + 3) % self.offset[self.orientation as usize].len() as u32
        }
    }
    pub fn move_down(&mut self) {
        if self.origin >= 21 {
            self.origin -= 11
        }
    }

    pub fn can_move(&mut self, m: &Matrix, val: i32) -> bool {
        for i in 0..self.offset[self.orientation as usize].len() {
            let index = self.origin as i32 + val + self.offset[self.orientation as usize][i as usize] as i32;
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
}
