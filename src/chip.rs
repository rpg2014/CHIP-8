use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;



pub struct Chip_8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    graphics_buffer: [u8; 64*32],
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    //keyboard: Keys,
    
}



impl Chip_8 {
    
    pub fn run_cycle(&self) -> Result<(), Box<dyn Error>> {
        let op_code: u16 = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[self.program_counter as usize + 1] as u16);

        //decode opcode
        //execute opcode
        Ok(())


    }

    pub fn init() -> Result<Chip_8, Box<dyn Error>> {
        let mut inital_memory: [u8; 4096] = [0; 4096];
        let chip8_font = Chip_8::get_font_array()?;
        for i in 0..80 {
            inital_memory[i] = chip8_font[i];
        }
        let chip8 = Chip_8  {
            memory: inital_memory,
            registers: [0; 16],
            program_counter: 0x200,
            graphics_buffer: [0; 64*32],
            index_register: 0,
            stack: Vec::with_capacity(16),
            delay_timer: 0,
            sound_timer: 0,
        };

        Ok(chip8)
    }

    // Loads the given file into the chip_8 memory starting at byte 0x200 (512)
    pub fn load_program(&mut self, file_name: String) -> Result<(), Box<dyn Error>> {
        let mut file: File = File::open(file_name)?;
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer)?;
        for i in 0..file_buffer.len() {
            self.memory[i + 512] = file_buffer[i];
        };
        Ok(())
    }

    pub fn get_graphics_buffer(&self) -> [u8; 64*32] {
        return self.graphics_buffer;
    }

    fn get_font_array() -> Result<Vec<u8>, Box<dyn Error>> {
        return Ok(vec![
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
]);
    }
}
