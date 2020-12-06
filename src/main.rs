mod chip_eight;
mod user_interface;
use chip_eight::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::Duration;
use user_interface::*;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

fn main() {
    let mut my_chip8: ChipEight;
    my_chip8 = ChipEight::new();
    let args: Vec<String> = env::args().collect();
    let scale: usize = 10;

    //println!("Path =  {}", args[1]);

    my_chip8.load_rom(&args[1]);  //Element 0 is the path to the .exe.  Element 1 is the path given when the program starts
    //my_chip8.load_rom("C:\\Repos\\SpaceInvaders[DavidWinter].ch8"); //This line is just to use for debug.  Not sure how to start the debugger with cmd arguments
    //my_chip8.load_rom("C:\\Repos\\Pong[PaulVervalin].ch8");
    //my_chip8.load_rom("C:\\Repos\\AstroDodge[RevivalStudios].ch8");

    let sdl_context = sdl2::init().unwrap();
    let mut my_user_interface = UserInterface::new(&sdl_context, scale);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut quit = false;

    while quit == false {
        //Emulation Cycle
        my_chip8.emulation_cycle();

        //render graphics
        my_user_interface.render(&my_chip8);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    quit = true;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => my_user_interface.key_press(&mut my_chip8, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => my_user_interface.key_release(&mut my_chip8, keycode),

                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}
