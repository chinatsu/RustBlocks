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

    fn clear_matrix(&mut self) {
        for i in 0..ARRAY_SIZE {
            if (i < REAL_WIDTH || i > ARRAY_SIZE || i % REAL_WIDTH == 0) {
                self.state[i as usize] = -1;
            } else {
                self.state[i as usize] = 0;
            }
        }
    }
}
