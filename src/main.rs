mod chip_eight;
mod user_interface;
use chip_eight::*;
use user_interface::*;

fn main() {
    let mut my_chip8: ChipEight;
    my_chip8 = ChipEight::new();
    my_chip8.load_rom();

    let mutmy_user_interface: UserInterface;

    loop {
        //Emulation Cycle
        my_chip8.emulation_cycle();
    }
}
