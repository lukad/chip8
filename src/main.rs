#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use std::env;

mod cpu;
mod instruction;
use cpu::Cpu;

fn main() {
    env_logger::init();

    let mut cpu = Cpu::new();

    if let Some(game_path) = env::args().nth(1) {
        cpu.load(game_path);
    }

    cpu.run();
}
