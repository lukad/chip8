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
    index: u16,
    pc: u16,
    sp: u16,
    memory: [u8; 4096],
    video: [bool; 2048],
    delay_timer: u8,
    sound_timer: u8,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Cpu {{ registers: {:?}, index: {:x}, pc: {:x}, sp: {:x}, stack: {:?}, delay: {:x}, sound: {:x}, opcode: {:x} }}",
            self.registers, self.index, self.pc, self.sp, self.stack, self.delay_timer, self.sound_timer, self.fetch()
        )
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: [0u8; 16],
            stack: [0u16; 16],
            index: 0,
            pc: 0x0200,
            sp: 0,
            memory: [0u8; 4096],
            video: [false; 2048],
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

    fn fetch(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16
    }

    fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x6000 => {
                // set VX to NN
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x] = nn;
                self.pc += 2;
            }
            _ => {
                debug!("{:?}", self);
                error!("opcode {:x} not implemented", opcode);
                std::process::exit(1);
            }
        }
    }

    pub fn run(&mut self) {
        debug!("Starting the emulation loop");
        loop {
            let opcode = self.fetch();
            self.execute(opcode);

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
