use crate::chip_eight::*;
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct UserInterface {
    canvas: WindowCanvas,
    scale: usize,
}

impl UserInterface {
    pub fn new(sdl_context: &sdl2::Sdl, size: usize) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "Chip8",
                (crate::DISPLAY_WIDTH * size) as u32,
                (crate::DISPLAY_HEIGHT * size) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut ui = UserInterface {
            canvas: window.into_canvas().build().unwrap(),
            scale: size,
        };

        ui.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ui.canvas.clear();
        ui.canvas.present();

        ui
    }

    pub fn key_press(&self, chip8: &mut ChipEight, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => {
                chip8.key[0x1] = true;
            }
            Keycode::Num2 => {
                chip8.key[0x2] = true;
            }
            Keycode::Num3 => {
                chip8.key[0x3] = true;
            }
            Keycode::Num4 => {
                chip8.key[0xC] = true;
            }
            Keycode::Q => {
                chip8.key[0x4] = true;
            }
            Keycode::W => {
                chip8.key[0x5] = true;
            }
            Keycode::E => {
                chip8.key[0x6] = true;
            }
            Keycode::R => {
                chip8.key[0xD] = true;
            }
            Keycode::A => {
                chip8.key[0x7] = true;
            }
            Keycode::S => {
                chip8.key[0x8] = true;
            }
            Keycode::D => {
                chip8.key[0x9] = true;
            }
            Keycode::F => {
                chip8.key[0xE] = true;
            }
            Keycode::Z => {
                chip8.key[0xA] = true;
            }
            Keycode::X => {
                chip8.key[0x0] = true;
            }
            Keycode::C => {
                chip8.key[0xB] = true;
            }
            Keycode::V => {
                chip8.key[0xF] = true;
            }
            _ => {}
        }
    }

    pub fn key_release(&self, chip8: &mut ChipEight, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => {
                chip8.key[0x1] = false;
            }
            Keycode::Num2 => {
                chip8.key[0x2] = false;
            }
            Keycode::Num3 => {
                chip8.key[0x3] = false;
            }
            Keycode::Num4 => {
                chip8.key[0xC] = false;
            }
            Keycode::Q => {
                chip8.key[0x4] = false;
            }
            Keycode::W => {
                chip8.key[0x5] = false;
            }
            Keycode::E => {
                chip8.key[0x6] = false;
            }
            Keycode::R => {
                chip8.key[0xD] = false;
            }
            Keycode::A => {
                chip8.key[0x7] = false;
            }
            Keycode::S => {
                chip8.key[0x8] = false;
            }
            Keycode::D => {
                chip8.key[0x9] = false;
            }
            Keycode::F => {
                chip8.key[0xE] = false;
            }
            Keycode::Z => {
                chip8.key[0xA] = false;
            }
            Keycode::X => {
                chip8.key[0x0] = false;
            }
            Keycode::C => {
                chip8.key[0xB] = false;
            }
            Keycode::V => {
                chip8.key[0xF] = false;
            }
            _ => {}
        }
    }

    pub fn render(&mut self, chip8: &ChipEight) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.clear();
        for i in 0..crate::DISPLAY_SIZE {
            let pixel = chip8.display[i];
            let x = i % crate::DISPLAY_WIDTH * self.scale; //get x position of pixel
            let y = i / crate::DISPLAY_WIDTH * self.scale; //get y position of pixel

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }

            let _ = self.canvas.fill_rect(Rect::new(
                x as i32,
                y as i32,
                self.scale as u32,
                self.scale as u32,
            )); //Draw the pixel as a square
        }

        self.canvas.present(); //display changes in window
    }
}
