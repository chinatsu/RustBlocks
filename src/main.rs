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
    matrix: Matrix
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BG:    [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let mut lines: Vec<[f64; 4]> = vec![];
        let mut columns: Vec<[f64; 4]> = vec![];
        for x in 0..(args.height/CELL_SIZE) {
            if x < 10 {
                columns.push([(x*CELL_SIZE) as f64, 0.0, (x*CELL_SIZE) as f64, args.height as f64]);
            }
            lines.push([0.0, (x*CELL_SIZE) as f64, args.width as f64, (x*CELL_SIZE) as f64]);
        }
        let ref mut piece = self.piece;
        let ref mut matrix = self.matrix;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG, gl);
            for l in lines {
                line(GREEN, 0.5, l, c.transform, gl);
            }
            for col in columns {
                line(GREEN, 0.5, col, c.transform, gl);
            }
            piece.draw(c, gl);
            matrix.draw(c, gl);

        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        if self.piece.soft_drop {
            self.piece.move_down(&mut self.matrix)
        }
        if self.piece.rot_l {
            self.piece.rotate(true);
        }
        if self.piece.rot_r {
            self.piece.rotate(false);
        }
        if self.piece.mov_left {
            if self.piece.can_move(&mut self.matrix, -1) {
                self.piece.origin -= 1;
            }
        }
        if self.piece.mov_right {
            if self.piece.can_move(&mut self.matrix, 1) {
                self.piece.origin += 1;
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
                self.piece.rot_l = true;
            }
            Key::Z => {
                self.piece.rot_r = true;
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
            }
            Key::Right => {
                self.piece.mov_right = false;
            }
            Key::X => {
                self.piece.rot_l = false;
            }
            Key::Z => {
                self.piece.rot_r = false;
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
            "spinning-square",
            [REAL_WIDTH*CELL_SIZE, REAL_HEIGHT*CELL_SIZE]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let pcs = &PIECES;
    let choice = rand::thread_rng().choose(pcs).unwrap();
    let p = Piece::new(0, *choice, [1.0, 1.0, 1.0, 0.5]);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        piece: p,
        matrix: Matrix::new()
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
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
