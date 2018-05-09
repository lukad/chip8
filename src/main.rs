#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;

struct Cpu {
    registers: [u8; 16],
    stack: [u16; 16],
    i: u16,
    pc: u16,
    sp: u16,
    opcode: u16,
    memory: [u8; 4096],
    vram: [u8; 2048],
    delay_timer: u8,
    sound_timer: u8,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f, "Cpu {{ registers: {:?}, index: {:x}, pc: {:x}, sp: {:x}, opcode: {:x}, stack: {:?}, delay: {:x}, sound: {:x} }}",
            self.registers,
            self.i,
            self.pc,
            self.sp,
            self.opcode,
            self.stack,
            self.delay_timer,
            self.sound_timer
        )
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: [0u8; 16],
            stack: [0u16; 16],
            i: 0,
            pc: 0x0200,
            sp: 0,
            opcode: 0,
            memory: [0u8; 4096],
            vram: [0u8; 2048],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load(&mut self, game_path: String) {
        info!("Loading {:?}", game_path);

        let mut f = File::open(game_path).expect("File not found");
        let n = f.read(&mut self.memory[0x0200..0x0FFF])
            .expect("Could not read game");

        debug!("Read {:?} bytes into memory", n);
    }

    fn fetch(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
    }

    fn nn(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    fn nnn(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    fn x(&self) -> u8 {
        ((self.opcode & 0x0F00) >> 8) as u8
    }

    fn y(&self) -> u8 {
        ((self.opcode & 0x00F0) >> 4) as u8
    }

    fn n(&self) -> u8 {
        ((self.opcode & 0x00F0) >> 4) as u8
    }

    fn execute(&mut self) {
        match self.opcode & 0xF000 {
            // 6XNN - Sets VX to NN
            0x6000 => {
                self.registers[self.x() as usize] = self.nn();
            }
            // ANNN - Sets I to NNN
            0xA000 => {
                self.i = self.nnn();
            }
            // DXYN - Draws sprite at (VX, VY) sized N*N pixels
            0xD000 => {
                let address = self.x() as usize + self.y() as usize * 32;
                for (i, pixel) in self.memory
                    [self.i as usize..(self.i as usize + self.n() as usize)]
                    .iter()
                    .enumerate()
                {
                    let old = self.vram[address + i];
                    let new = old ^ pixel;
                    self.vram[address] = new;
                    if old == 1 && new == 0 {
                        self.registers[0xF] = 1;
                    }
                }
            }
            _ => {
                debug!("{:?}", self);
                error!("self.opcode {:x} not implemented", self.opcode);
                std::process::exit(1);
            }
        }
        self.pc += 2;
    }

    pub fn run(&mut self) {
        debug!("Starting the emulation loop");
        loop {
            self.fetch();
            self.execute();

            // update timers
        }
    }
}

fn main() {
    env_logger::init();

    let mut cpu = Cpu::new();

    if let Some(game_path) = env::args().nth(1) {
        cpu.load(game_path);
    }

    cpu.run();
}
