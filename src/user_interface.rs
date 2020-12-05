use crate::chip_eight::*;
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct UserInterface {
    canvas: WindowCanvas,
}

impl UserInterface {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip8", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut ui = UserInterface {
            canvas: window.into_canvas().build().unwrap(),
        };

        ui.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ui.canvas.clear();
        ui.canvas.present();

        ui
    }

    pub fn render(&mut self, chip8: &ChipEight, scale: u32) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.clear();
        for i in 0..crate::DISPLAY_SIZE {
            let pixel = chip8.display[i];
            let x = i % crate::DISPLAY_WIDTH * scale as usize; //get x position of pixel
            let y = i / crate::DISPLAY_WIDTH * scale as usize; //get y position of pixel

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }

            let _ = self
                .canvas
                .fill_rect(Rect::new(x as i32, y as i32, scale, scale)); //Draw the pixel as a square
        }

        self.canvas.present(); //display changes in window
    }
}
