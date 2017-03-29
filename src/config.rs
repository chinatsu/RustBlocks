extern crate rustc_serialize;
use rustc_serialize::{Encodable, Decodable};

#[derive(RustcEncodable, RustcDecodable, Debug, Copy, Clone)]
pub struct Config {
    pub gameplay: GameplayConfig,
    pub orientations: OrientationConfig
}

#[derive(RustcEncodable, RustcDecodable, Debug, Copy, Clone)]
pub struct GameplayConfig {
    pub das: u64
}

#[derive(RustcEncodable, RustcDecodable, Debug, Copy, Clone)]
pub struct OrientationConfig {
    pub i: u32,
    pub o: u32,
    pub t: u32,
    pub s: u32,
    pub z: u32,
    pub j: u32,
    pub l: u32
}

impl Default for Config {
    fn default() -> Config {
        Config {
            gameplay: GameplayConfig::default(),
            orientations: OrientationConfig::default()
        }
    }
}

impl Default for GameplayConfig {
    fn default() -> GameplayConfig {
        GameplayConfig {
            das: 15
        }
    }
}

impl Default for OrientationConfig {
    fn default() -> OrientationConfig {
        OrientationConfig {
            i: 0,
            o: 0,
            t: 0,
            s: 0,
            z: 0,
            j: 0,
            l: 0
        }
    }
}
