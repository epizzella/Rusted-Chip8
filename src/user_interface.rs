use chip_eight::*;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct UserInterface {
    canvas: WindowCanvas,
}

impl UserInterface {
    pub fn render(&mut self, chip8: ChipEight, scale: u32) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        self.canvas.clear();

        self.canvas.present();
    }
}
