use cpu::Cpu;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::{Duration, Instant};

pub struct Chip8 {
    cpu: Cpu,
    canvas: sdl2::render::WindowCanvas,
    events: sdl2::EventPump,
}

const FONT: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xf0, 0x10, 0xf0, 0x80, 0xf0, 0xf0,
    0x10, 0xf0, 0x10, 0xf0, 0x90, 0x90, 0xf0, 0x10, 0x10, 0xf0, 0x80, 0xf0, 0x10, 0xf0, 0xf0, 0x80,
    0xf0, 0x90, 0xf0, 0xf0, 0x10, 0x20, 0x40, 0x40, 0xf0, 0x90, 0xf0, 0x90, 0xf0, 0xf0, 0x90, 0xf0,
    0x10, 0xf0, 0xf0, 0x90, 0xf0, 0x90, 0x90, 0xe0, 0x90, 0xe0, 0x90, 0xe0, 0xf0, 0x80, 0x80, 0x80,
    0x80, 0xe0, 0x90, 0x90, 0x90, 0xe0, 0xf0, 0x80, 0xf0, 0x80, 0xf0, 0xf0, 0x80, 0xf0, 0x80, 0x80,
];

impl Chip8 {
    pub fn new() -> Chip8 {
        debug!("Creating SDL2 context");
        let sdl_context = sdl2::init().unwrap();

        debug!("Initializing SDL2 video subsystem");
        let video = sdl_context.video().unwrap();

        debug!("Creating a window");
        let window = video
            .window("CHIP-8", 768, 384)
            .position_centered()
            .build()
            .unwrap();

        debug!("Creating a canvas");
        let canvas = window.into_canvas().build().unwrap();

        debug!("Initializing SDL2 event pump");
        let events = sdl_context.event_pump().unwrap();

        let mut cpu = Cpu::new();
        cpu.load_font(FONT);

        Chip8 {
            cpu: cpu,
            canvas: canvas,
            events: events,
        }
    }

    pub fn load(&mut self, path: String) {
        self.cpu.load(path);
    }

    pub fn run(&mut self) {
        let mut cpu_instant = Instant::now();
        let mut frame_instant = Instant::now();
        let mut cpu_error = false;

        debug!("Starting the emulation loop");
        'outer: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'outer,
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Num1) => self.cpu.keys[0x1] = 1,
                        Some(Keycode::Num2) => self.cpu.keys[0x2] = 1,
                        Some(Keycode::Num3) => self.cpu.keys[0x3] = 1,
                        Some(Keycode::Num4) => self.cpu.keys[0xC] = 1,
                        Some(Keycode::Q) => self.cpu.keys[0x4] = 1,
                        Some(Keycode::W) => self.cpu.keys[0x5] = 1,
                        Some(Keycode::E) => self.cpu.keys[0x6] = 1,
                        Some(Keycode::R) => self.cpu.keys[0xD] = 1,
                        Some(Keycode::A) => self.cpu.keys[0x7] = 1,
                        Some(Keycode::S) => self.cpu.keys[0x8] = 1,
                        Some(Keycode::D) => self.cpu.keys[0x9] = 1,
                        Some(Keycode::F) => self.cpu.keys[0xE] = 1,
                        Some(Keycode::Z) => self.cpu.keys[0xA] = 1,
                        Some(Keycode::X) => self.cpu.keys[0x0] = 1,
                        Some(Keycode::C) => self.cpu.keys[0xB] = 1,
                        Some(Keycode::V) => self.cpu.keys[0xF] = 1,
                        _ => (),
                    },
                    Event::KeyUp { keycode, .. } => match keycode {
                        Some(Keycode::Num1) => self.cpu.keys[0x1] = 0,
                        Some(Keycode::Num2) => self.cpu.keys[0x2] = 0,
                        Some(Keycode::Num3) => self.cpu.keys[0x3] = 0,
                        Some(Keycode::Num4) => self.cpu.keys[0xC] = 0,
                        Some(Keycode::Q) => self.cpu.keys[0x4] = 0,
                        Some(Keycode::W) => self.cpu.keys[0x5] = 0,
                        Some(Keycode::E) => self.cpu.keys[0x6] = 0,
                        Some(Keycode::R) => self.cpu.keys[0xD] = 0,
                        Some(Keycode::A) => self.cpu.keys[0x7] = 0,
                        Some(Keycode::S) => self.cpu.keys[0x8] = 0,
                        Some(Keycode::D) => self.cpu.keys[0x9] = 0,
                        Some(Keycode::F) => self.cpu.keys[0xE] = 0,
                        Some(Keycode::Z) => self.cpu.keys[0xA] = 0,
                        Some(Keycode::X) => self.cpu.keys[0x0] = 0,
                        Some(Keycode::C) => self.cpu.keys[0xB] = 0,
                        Some(Keycode::V) => self.cpu.keys[0xF] = 0,
                        _ => (),
                    },
                    _ => (),
                }
            }

            if !cpu_error && cpu_instant.elapsed() >= Duration::from_millis(2) {
                match self.cpu.step() {
                    Err(error) => {
                        error!("CPU encountered an error: {:?}", error);
                        cpu_error = true;
                    }
                    _ => (),
                }
                cpu_instant = Instant::now();
            }

            if frame_instant.elapsed() >= Duration::from_millis(16) {
                self.draw();
                frame_instant = Instant::now();
            }
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for y in 0..32 {
            for x in 0..64 {
                if self.cpu.vram[y * 64 + x] & 1 != 1 {
                    continue;
                }
                self.canvas
                    .fill_rect(Rect::new(x as i32 * 12, y as i32 * 12, 12, 12))
                    .unwrap();
            }
        }

        self.canvas.present();
    }
}
