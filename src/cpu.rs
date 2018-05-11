use std::fmt;
use std::fs::File;
use std::io::Read;
use std::process;

use instruction::*;

pub struct Cpu {
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

pub struct Opcode(pub u8, pub u8);

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
    pub fn new() -> Cpu {
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

    pub fn run(&mut self) {
        debug!("Starting the emulation loop");
        loop {
            let opcode = self.fetch();
            let instruction = Instruction::decode(opcode);
            self.execute(instruction);

            // update timers
        }
    }

    fn fetch(&mut self) -> Opcode {
        Opcode(
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        )
    }

    fn execute(&mut self, instruction: Instruction) {
        trace!("{:?}", instruction);

        let mut increment_pc = true;

        match instruction {
            Call(address) => {
                self.stack[self.sp as usize] = self.pc;
                self.pc = address;
                increment_pc = false;
            }
            LoadConstant(x, kk) => self.registers[x as usize] = kk,
            AddConstant(x, kk) => self.registers[x as usize] += kk,
            SetAddress(address) => self.i = address,
            Draw(x, y, n) => {
                let address = (self.registers[x as usize] + self.registers[y as usize]) as usize;
                for (i, pixel) in self.memory[self.i as usize..=(self.i as usize + n as usize)]
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
            SetFontLocation(x) => self.i = self.registers[x as usize] as u16 * 5,
            SetBCD(x) => {
                self.memory[self.i as usize] = self.registers[x as usize] / 100;
                self.memory[self.i as usize + 1] = self.registers[x as usize] % 100 / 10;
                self.memory[self.i as usize + 2] = self.registers[x as usize] % 100 % 10;
                self.pc += 2;
            }
            _ => {
                error!("Instruction not implemented: {:?}", instruction);
                process::exit(1);
            }
        }

        if increment_pc {
            self.pc += 2;
        }
    }
}
