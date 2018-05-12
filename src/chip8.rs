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
        let sdl_context = sdl2::init().expect("Could not create SDL2 context");
        debug!("Initializing SDL2 video subsystem");
        let video = sdl_context
            .video()
            .expect("Could not initialize SDL2 video subsystem");
        debug!("Creating a window");
        let window = video
            .window("CHIP-8", 768, 384)
            .position_centered()
            .build()
            .expect("Could not create window");
        debug!("Creating a canvas");
        let canvas = window
            .into_canvas()
            .build()
            .expect("Could not create canvas");
        debug!("Initializing SDL2 event pump");
        let events = sdl_context
            .event_pump()
            .expect("Could initialize event subsystem");

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
                    Event::Quit { .. } => break 'outer,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'outer,
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
        let white = Color::RGB(255, 255, 255);

        for y in 0..32 {
            for x in 0..64 {
                let bg_color = match (x % 2, y % 2) {
                    (0, 1) | (1, 0) => Color::RGB(80, 80, 80),
                    (_, _) => Color::RGB(20, 20, 20),
                };
                let color = match self.cpu.vram[y * 64 + x] & 1 {
                    1 => white,
                    _ => bg_color,
                };
                self.canvas.set_draw_color(color);
                self.canvas
                    .fill_rect(Rect::new(x as i32 * 12, y as i32 * 12, 12, 12))
                    .unwrap();
            }
        }

        self.canvas.present();
    }
}
