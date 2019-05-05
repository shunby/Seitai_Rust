extern crate lazy_static;
extern crate gdk_pixbuf;
extern crate rand;

use lazy_static::lazy_static;

use gdk_pixbuf::Pixbuf;

use std::sync::*;
use std::cell::*;
use std::fs::File;
use std::io::Read;

use super::world::world::World;
use std::time::{Duration, Instant};
use std::collections::HashMap;

//static vars
lazy_static! {
    pub static ref GAME: RwLock<Game> = {
        let mut g = Game::new();
        let mut file = File::open("grass.png").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        g.lib_pict.insert("grass",buf);
        RwLock::new(g)
    };
}

pub struct Game{
    pub time: u128,
    pub last_time : Instant,
    pub world: World,
    pub lib_pict : HashMap<&'static str, Vec<u8>>
}

impl Game{
    pub fn new() -> Game{
        Game{time: 0, world: World::new(), last_time: Instant::now(), lib_pict: HashMap::new()}
    }
}