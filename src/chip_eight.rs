use num_derive::FromPrimitive;

use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use strum_macros::AsRefStr;

//use rand::prelude::*;

const SPRITE_START_ADDR: usize = 0x50;
const SUB_OPCODE_MASK: u16 = 0x000F;
const SUB_OPCODE_MASK2: u16 = 0x00FF;
const OPCODE_MASK: u16 = 0xF000;
const ADDR_MASK: u16 = 0x0FFF;
const BYTE_MASK: u16 = 0x00FF;
const NIBBLE_MASK: u16 = 0x000F;

#[derive(AsRefStr, Debug)]
enum Opcodes {
    ClearOrReturn(u16),            //sub code
    Jump(usize),                   //address
    Call(usize),                   //address
    SkipEqual(usize, u8),          //Vx, K
    SkipNotEqual(usize, u8),       //Vx, K
    SkipEqualVy(usize, usize),     //Vx, Vy
    LoadVxK(usize, u8),            //Vx, K
    AddByte(usize, u8),            //Vx, K
    Arithmetic(u16, usize, usize), //sub code, Vx, Vy
    SkipNotEqualVy(usize, usize),  //Vx, Vy,
    LoadI(usize),                  //address,
    JumpOffset(usize),             //address,
    RandomVxByte(usize, u8),       //Vx, K,
    Draw(u16, u16, u16),           //Vx, Vy, Height
    SkipPressed(usize, u16),       //Vx, sub code
    Misc(usize),                   //Vx
    BadOpcode,
}

pub struct ChipEight {
    opcode: u16, //op code is two bytes long
    //memory map
    //0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    //0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    //0x200-0xFFF - Program ROM and work RAM
    memory: [u8; 4096], //4k memmory

    v_register: [u8; 16], //16 registers; v0 - vF
    i_register: usize,    //index register
    pc: usize,            //program counter register

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16], //The stack
    sp: usize,        //The stack pointer

    key: [u8; 16],
    pub display: [u8; crate::DISPLAY_SIZE], //Chip8 has a display that is 64 x 32
}

#[derive(FromPrimitive)]
enum SubOpcodeClearorReturn {
    Clear = 0x0,
    Return = 0xE,
}

impl ChipEight {
    pub fn new() -> Self {
        let mut chip8 = Self {
            opcode: 0,
            memory: [0; 4096],
            v_register: [0; 16],
            i_register: 0,
            pc: 0x200, //program starts at 0x200
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            display: [0; crate::DISPLAY_SIZE],
        };

        const FONT_SIZE: usize = 80;
        let font: [u8; FONT_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        //loading hardcoded fonts into memory
        for x in 0..FONT_SIZE {
            chip8.memory[SPRITE_START_ADDR + x] = font[x]
        }

        chip8
    }

    //Returns index for V[X] from opcode
    fn vx_mask(opcode: u16) -> usize {
        const VX_MASK: u16 = 0x0F00;
        ((opcode & VX_MASK) >> 8) as usize
    }

    //Returns index for V[Y] from opcode
    fn vy_mask(opcode: u16) -> usize {
        const VY_MASK: u16 = 0x00F0;
        ((opcode & VY_MASK) >> 4) as usize
    }

    fn fetch(&mut self) -> Opcodes {
        let opcode: Opcodes;

        self.opcode = (self.memory[self.pc] as u16) << 8; //op code is two bytes long
        self.opcode |= (self.memory[self.pc + 1]) as u16;

        match self.opcode & OPCODE_MASK {
            0x0000 => opcode = Opcodes::ClearOrReturn(self.opcode & SUB_OPCODE_MASK),
            0x1000 => opcode = Opcodes::Jump((self.opcode & ADDR_MASK) as usize),
            0x2000 => opcode = Opcodes::Call((self.opcode & ADDR_MASK) as usize),
            0x3000 => {
                opcode =
                    Opcodes::SkipEqual(Self::vx_mask(self.opcode), (self.opcode & BYTE_MASK) as u8)
            }
            0x4000 => {
                opcode = Opcodes::SkipNotEqual(
                    Self::vx_mask(self.opcode),
                    (self.opcode & BYTE_MASK) as u8,
                ) // I want to punch visual studio code / rust for auto formatting this line.  Monitors aren't square anymore!!!
            }
            0x5000 => {
                opcode =
                    Opcodes::SkipEqualVy(Self::vx_mask(self.opcode), Self::vy_mask(self.opcode))
            }
            0x6000 => {
                opcode =
                    Opcodes::LoadVxK(Self::vx_mask(self.opcode), (self.opcode & BYTE_MASK) as u8)
            }
            0x7000 => {
                opcode =
                    Opcodes::AddByte(Self::vx_mask(self.opcode), (self.opcode & BYTE_MASK) as u8)
            }
            0x8000 => {
                opcode = Opcodes::Arithmetic(
                    self.opcode & SUB_OPCODE_MASK,
                    Self::vx_mask(self.opcode),
                    Self::vy_mask(self.opcode),
                )
            }
            0x9000 => {
                opcode =
                    Opcodes::SkipNotEqualVy(Self::vx_mask(self.opcode), Self::vy_mask(self.opcode))
            }
            0xA000 => opcode = Opcodes::LoadI((self.opcode & ADDR_MASK) as usize),
            0xB000 => opcode = Opcodes::JumpOffset((self.opcode & ADDR_MASK) as usize),
            0xC000 => {
                opcode = Opcodes::RandomVxByte(
                    Self::vx_mask(self.opcode),
                    (self.opcode & BYTE_MASK) as u8,
                )
            }
            0xD000 => {
                opcode = Opcodes::Draw(
                    self.v_register[Self::vx_mask(self.opcode)] as u16,
                    self.v_register[Self::vy_mask(self.opcode)] as u16,
                    self.opcode & NIBBLE_MASK,
                )
            }
            0xE000 => {
                opcode =
                    Opcodes::SkipPressed(Self::vx_mask(self.opcode), self.opcode & SUB_OPCODE_MASK)
            }
            0xF000 => opcode = Opcodes::Misc(Self::vx_mask(self.opcode)),
            _ => opcode = Opcodes::BadOpcode,
        }

        opcode
    }

    pub fn emulation_cycle(&mut self) {
        //Fetch opcode
        let opcode = self.fetch();
        self.pc += 2; //increment the pc for the next instruction

        println!("Opcode: {}", opcode.as_ref());

        //Decode opcode
        match opcode {
            Opcodes::ClearOrReturn(sub) => {
                match sub {
                    //Clear Display
                    0x0000 => {
                        println!("----- Clear Display");
                        for i in 0..crate::DISPLAY_SIZE {
                            self.display[i] = 0;
                        }
                    }
                    //00EE: Return from subroutine
                    0x000E => {
                        println!("----- Return");
                        self.sp -= 1;
                        self.pc = self.stack[self.sp] as usize; //pop program counter off of the stack
                    }
                    _ => {
                        println!("----- other");
                    }
                }
            }
            Opcodes::Jump(addr) => {
                self.pc = addr; //jump to the address
            }
            Opcodes::Call(addr) => {
                self.stack[self.sp] = self.pc as u16; //Push the program counter onto the stack
                self.sp += 1;
                self.pc = addr; //jump to the address
            }
            Opcodes::SkipEqual(vx, k) => {
                if self.v_register[vx] == k {
                    self.pc += 2;
                }
            }
            Opcodes::SkipNotEqual(vx, k) => {
                if self.v_register[vx] != k {
                    self.pc += 2;
                }
            }
            Opcodes::SkipEqualVy(vx, vy) => {
                if self.v_register[vx] == self.v_register[vy] {
                    self.pc += 2;
                }
            }
            Opcodes::LoadVxK(vx, k) => {
                self.v_register[vx] = k;
            }
            Opcodes::AddByte(vx, k) => {
                self.v_register[vx] = self.v_register[vx].wrapping_add(k);
            }
            Opcodes::Arithmetic(sub, vx, vy) => {
                match sub {
                    //Assign
                    0x0000 => {
                        self.v_register[vx] = self.v_register[vy];
                    }
                    //Bitwise OR
                    0x0001 => {
                        self.v_register[vx] |= self.v_register[vy];
                    }
                    //Bitewise AND
                    0x0002 => {
                        self.v_register[vx] &= self.v_register[vy];
                    }
                    //Bitwise XOR
                    0x0003 => {
                        self.v_register[vx] ^= self.v_register[vy];
                    }
                    //Add Vy to Vx.  If sum is greater than 255 mark v[F] as 1
                    0x0004 => {
                        if self.v_register[vx] > 0xFF - self.v_register[Self::vx_mask(self.opcode)]
                        {
                            self.v_register[0xF] = 1;
                        } else {
                            self.v_register[0xF] = 0;
                        }
                        self.v_register[vx] = self.v_register[vx].wrapping_add(self.v_register[vy]);
                    }
                    //Sub
                    0x0005 => {
                        if self.v_register[vx] > self.v_register[vy] {
                            self.v_register[0xF] = 1;
                        } else {
                            self.v_register[0xF] = 0;
                        }
                        self.v_register[vx] = self.v_register[vx].wrapping_sub(self.v_register[vy]);
                    }
                    //Right Shift
                    0x0006 => {
                        self.v_register[0xF] = self.v_register[vx] & 0x01;
                        self.v_register[vx] >>= 1;
                    }
                    //sub
                    0x0007 => {
                        if self.v_register[vy] > self.v_register[vx] {
                            self.v_register[0xF] = 1;
                        } else {
                            self.v_register[0xF] = 0;
                        }
                        self.v_register[vx] = self.v_register[vy] - self.v_register[vx];
                    }
                    //Left Shift
                    0x000E => {
                        self.v_register[0xF] = self.v_register[vx] & 0x10;
                        self.v_register[vx] <<= 1;
                    }
                    _ => (),
                }
            }
            Opcodes::SkipNotEqualVy(vx, vy) => {
                if self.v_register[vx] != self.v_register[vy] {
                    self.pc += 2;
                }
            }
            Opcodes::LoadI(addr) => {
                self.i_register = addr;
            }
            Opcodes::JumpOffset(addr) => {
                self.pc = addr + self.v_register[0] as usize;
            }
            Opcodes::RandomVxByte(vx, k) => {
                self.v_register[vx] = rand::random::<u8>() & k;
            }
            Opcodes::Draw(vx, vy, height) => {
                self.v_register[0xF] = 0;

                let x_pos = (vx as usize % crate::DISPLAY_WIDTH) as u16;
                let y_pos = (vy as usize % crate::DISPLAY_HEIGHT) as u16;

                for y in 0..height {
                    let pixel = self.memory[(self.i_register + y as usize) as usize]; //get the pixel value from the sprite stored in memory

                    for x in 0..8 {
                        if (pixel & (0x80 >> x)) != 0 {
                            let current_position = ((x_pos + x)
                                + ((y_pos + y) * crate::DISPLAY_WIDTH as u16))
                                as usize; //Treats display as though it were a 2d array

                            if self.display[current_position] == 1 {
                                self.v_register[0xF] = 1;
                            }
                            self.display[current_position as usize] ^= 1;
                        }
                    }
                }
            }
            Opcodes::SkipPressed(vx, sub) => match sub {
                //Skips the next instruction if the key in Vx is not pressed
                0x1 => {
                    if self.key[self.v_register[vx] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                //Skips the next instruction if the key in Vx is pressed
                0xE => {
                    if self.key[self.v_register[vx] as usize] > 0 {
                        self.pc += 2;
                    }
                }
                _ => {
                    println!("Skip Pressed sub code does not exist: {} ", sub)
                }
            },
            Opcodes::Misc(vx) => {
                let subcode = self.opcode & SUB_OPCODE_MASK2;

                match subcode {
                    //load delay timer
                    0x0007 => {
                        self.v_register[vx] = self.delay_timer;
                    }
                    //wait for key press and save it in Vx
                    0x000A => {
                        if self.key[0] > 0 {
                            self.v_register[vx] = 0;
                        } else if self.key[1] > 0 {
                            self.v_register[vx] = 1;
                        } else if self.key[2] > 0 {
                            self.v_register[vx] = 2;
                        } else if self.key[3] > 0 {
                            self.v_register[vx] = 3;
                        } else if self.key[4] > 0 {
                            self.v_register[vx] = 4;
                        } else if self.key[5] > 0 {
                            self.v_register[vx] = 5;
                        } else if self.key[6] > 0 {
                            self.v_register[vx] = 6;
                        } else if self.key[7] > 0 {
                            self.v_register[vx] = 7;
                        } else if self.key[8] > 0 {
                            self.v_register[vx] = 8;
                        } else if self.key[9] > 0 {
                            self.v_register[vx] = 9;
                        } else if self.key[10] > 0 {
                            self.v_register[vx] = 10;
                        } else if self.key[11] > 0 {
                            self.v_register[vx] = 11;
                        } else if self.key[12] > 0 {
                            self.v_register[vx] = 12;
                        } else if self.key[13] > 0 {
                            self.v_register[vx] = 13;
                        } else if self.key[14] > 0 {
                            self.v_register[vx] = 14;
                        } else if self.key[15] > 0 {
                            self.v_register[vx] = 15;
                        } else {
                            //So a pretty easy to wait to just to repeat the same instruction
                            self.pc -= 2;
                        }
                    }
                    0x0015 => {
                        self.delay_timer = self.v_register[vx];
                    }
                    0x0018 => {
                        self.sound_timer = self.v_register[vx];
                    }
                    0x001E => {
                        self.i_register += self.v_register[vx] as usize;
                    }
                    0x0029 => {
                        //Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                        //Each sprite is 5 bytes tall
                        self.i_register = SPRITE_START_ADDR + (5 * self.v_register[vx]) as usize;
                    }
                    0x0033 => {
                        //FX33: Stores the binary-coded decimal representation of VX,
                        self.memory[self.i_register] = self.v_register[vx] / 100;
                        self.memory[self.i_register + 1] = (self.v_register[vx] / 10) % 10;
                        self.memory[self.i_register + 2] = (self.v_register[vx] % 100) % 10;
                    }
                    0x0055 => {
                        //Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
                        for x in 0..vx + 1 {
                            self.memory[self.i_register + x] = self.v_register[x];
                        }
                    }
                    0x0065 => {
                        for x in 0..vx + 1 {
                            self.v_register[x] = self.memory[self.i_register + x];
                        }
                    }
                    _ => {
                        println!("Fxxx Opcode bad subcode: {}", subcode)
                    } //Call machine code routine
                }
            }
            Opcodes::BadOpcode => println!("Invalid opcode {}", self.opcode),
        }

        //update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn load_rom(&mut self, file_path: &str) {
        let mut rom = File::open(file_path).expect("Rom was not found");
        let mut buffer = [0; 3584];
        let buffer_size = rom.read(&mut buffer[..]).expect("Error when reading file");
        for i in 0..buffer_size {
            self.memory[i + 512] = buffer[i];
        }
    }
}
