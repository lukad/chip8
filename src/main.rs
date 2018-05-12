#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
extern crate sdl2;

use std::env;

mod chip8;
mod cpu;
mod instruction;

use chip8::Chip8;

fn main() {
    env_logger::init();

    let mut chip8 = Chip8::new();

    if let Some(game_path) = env::args().nth(1) {
        chip8.load(game_path);
    }

    chip8.run();
}
