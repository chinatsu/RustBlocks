extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate toml_config;
extern crate rustc_serialize;
extern crate find_folder;
pub mod tetris;
pub mod piece;
pub mod config;


use std::path::Path;
use toml_config::ConfigFactory;
use rustc_serialize::{Encodable, Decodable};
use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use tetris::*;
use piece::*;
use config::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use rand::Rng;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    piece: Piece,
    matrix: Matrix,
    config: Config,
    das: u64,
    gravity: u64,
    moved_right: bool,
    moved_left: bool,
    time: f64,
    glyphs: GlyphCache<'static>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BG:    [f32; 4] = [0.1, 0.1, 0.1, 1.0];


        let mut lines: Vec<[f64; 4]> = vec![];
        let mut columns: Vec<[f64; 4]> = vec![];
        for x in 0..(args.height/CELL_SIZE) {
            if x < REAL_WIDTH {
                columns.push([(x*CELL_SIZE) as f64, 0.0, (x*CELL_SIZE) as f64, args.height as f64]);
            }
            lines.push([0.0, (x*CELL_SIZE) as f64, (WIDTH * CELL_SIZE) as f64, (x*CELL_SIZE) as f64]);
        }
        let ref mut use_cache = self.glyphs;
        let ref mut piece = self.piece;
        let ref mut matrix = self.matrix;

        let mut text = graphics::Text::new(22);
        text.color = [1.0, 1.0, 1.0, 1.0];
        let pos = piece.origin;
        self.gl.draw(args.viewport(), |c, gl| {
            let mut transform: graphics::context::Context = c.trans(330.0, 160.0);
            // Clear the screen.
            clear(BG, gl);
            for l in lines {
                line(GREEN, 0.5, l, c.transform, gl);
            }
            for col in columns {
                line(GREEN, 0.5, col, c.transform, gl);
            }
            // Draw ghost
            while piece.can_move(matrix, -(REAL_WIDTH as i32)) {
                piece.origin -= REAL_WIDTH;
            }
            piece.draw(8, c, gl);
            piece.origin = pos;

            let id = piece.id as i32;
            piece.draw(id, c, gl);
            matrix.draw(c, gl);
            piece.draw_next(c, gl);

            text.draw(&format!("Lines: {}", matrix.lines_cleared), use_cache, &c.draw_state, transform.transform, gl)
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.gravity += 1;

        if self.gravity % self.config.gameplay.gravity == 0 {
            self.gravity = 0;
            let _ = self.piece.move_down(&mut self.matrix);
        }
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
            self.das += 1;
            if self.das > self.config.gameplay.das {
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
            self.das += 1;
            if self.das > self.config.gameplay.das {
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
                if self.piece.mov_right == true {
                    self.piece.mov_right == false;
                    self.das = 0;
                }
                self.piece.mov_left = true;
            }
            Key::Right => {
                if self.piece.mov_left == true {
                    self.piece.mov_left == false;
                    self.das = 0;
                }
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
                self.das = 0;
                self.moved_left = false;
            }
            Key::Right => {
                self.piece.mov_right = false;
                self.das = 0;
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets").unwrap();
    let ref font = assets.join("FiraMono-Regular.ttf");

    let pcs = &PIECES;
    let choice = rand::thread_rng().gen_range(0, 7);
    let p = Piece::new(choice, pcs[choice as usize]);
    let mut app = App {
        gl: GlGraphics::new(opengl),
        piece: p,
        matrix: Matrix::new(),
        config: ConfigFactory::load(Path::new("config.toml")),
        das: 0,
        gravity: 0,
        moved_right: false,
        moved_left: false,
        time: 0.0,
        glyphs: GlyphCache::new(font).unwrap()
    };

    let mut events = Events::new(EventSettings::new()).max_fps(120);
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
