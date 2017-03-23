extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
pub mod tetris;
pub mod piece;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use tetris::*;
use piece::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::Rng;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    piece: Piece,
    matrix: Matrix,
    das_right: u64,
    moved_right: bool,
    das_left: u64,
    moved_left: bool
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BG:    [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let mut lines: Vec<[f64; 4]> = vec![];
        let mut columns: Vec<[f64; 4]> = vec![];
        for x in 0..(args.height/CELL_SIZE) {
            if x < 11 {
                columns.push([(x*CELL_SIZE) as f64, 0.0, (x*CELL_SIZE) as f64, args.height as f64]);
            }
            lines.push([0.0, (x*CELL_SIZE) as f64, (WIDTH * CELL_SIZE) as f64, (x*CELL_SIZE) as f64]);
        }
        let ref mut piece = self.piece;
        let ref mut matrix = self.matrix;
        let pos = piece.origin;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);
            for l in lines {
                line(GREEN, 0.5, l, c.transform, gl);
            }
            for col in columns {
                line(GREEN, 0.5, col, c.transform, gl);
            }
            // Draw ghost
            while piece.can_move(matrix, -11) {
                piece.origin -= 11;
            }
            piece.draw(8, c, gl);
            piece.origin = pos;

            let id = piece.id as i32;
            piece.draw(id, c, gl);
            matrix.draw(c, gl);
            piece.draw_next(c, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        if self.piece.soft_drop {
            let _ = self.piece.move_down(&mut self.matrix);
        }
        if self.piece.mov_left {
            if !self.moved_left {
                if self.piece.can_move(&mut self.matrix, -1) {
                    self.piece.origin -= 1;
                }
                self.moved_left = true;
            }
            self.das_left += 1;
            if self.das_left > 150 {
                if self.piece.can_move(&mut self.matrix, -1) {
                    self.piece.origin -= 1;
                }
            }
        }
        if self.piece.mov_right {
            if !self.moved_right {
                if self.piece.can_move(&mut self.matrix, 1) {
                    self.piece.origin += 1;
                }
                self.moved_right = true;
            }
            self.das_right += 1;
            if self.das_right > 150 {
                if self.piece.can_move(&mut self.matrix, 1) {
                    self.piece.origin += 1;
                }
            }
        }
    }

    fn on_press(&mut self, key: keyboard::Key) {
        match key {
            Key::Down => {
                self.piece.soft_drop = true;
            }
            Key::Left => {
                self.piece.mov_left = true;
            }
            Key::Right => {
                self.piece.mov_right = true;
            }
            Key::X => {
                if !self.piece.rot {
                    self.piece.rotate(&mut self.matrix, 1);
                    self.piece.rot = true;
                }
            }
            Key::Z => {
                if !self.piece.rot {
                    self.piece.rotate(&mut self.matrix, 3);
                    self.piece.rot = true;
                }
            }
            Key::C => {
                if !self.piece.rot {
                    self.piece.rotate(&mut self.matrix, 2);
                    self.piece.rot = true;
                }
            }
            Key::Space => {
                if !self.piece.hard_drop {
                    self.piece.hard_drop(&mut self.matrix);
                    self.piece.hard_drop = true;
                }
            }
            _ => {}
        }
    }
    fn on_release(&mut self, key: keyboard::Key) {
        match key {
            Key::Down => {
                self.piece.soft_drop = false;
            }
            Key::Left => {
                self.piece.mov_left = false;
                self.das_left = 0;
                self.moved_left = false;
            }
            Key::Right => {
                self.piece.mov_right = false;
                self.das_right = 0;
                self.moved_right = false;
            }
            Key::X => {
                self.piece.rot = false;
            }
            Key::Z => {
                self.piece.rot = false;
            }
            Key::C => {
                self.piece.rot = false;
            }
            Key::Space => {
                self.piece.hard_drop = false;
            }
            _ => {}
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "rustblocks",
            [(WIDTH+10)*CELL_SIZE, HEIGHT*CELL_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let pcs = &PIECES;
    let choice = rand::thread_rng().gen_range(0, 7);
    let p = Piece::new(choice, 0, pcs[choice as usize], [1.0, 1.0, 1.0, 0.5]);
    let mut app = App {
        gl: GlGraphics::new(opengl),
        piece: p,
        matrix: Matrix::new(),
        das_right: 0,
        moved_right: false,
        das_left: 0,
        moved_left: false
    };

    let mut events = Events::new(EventSettings::new()).ups(1000);
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.on_press(key);
        };
        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.on_release(key);
        };
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
