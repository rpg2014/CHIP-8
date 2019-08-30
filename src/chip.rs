use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;
use std::collections::HashSet;
use crate::keyboard::Keyboard;
use string_error::static_err;



pub struct Chip_8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    graphics_buffer: [u8; 64*32],
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    keyboard: Keyboard,
    
}



impl Chip_8 {
    
    pub fn run_cycle(&mut self) -> Result<(), Box<dyn Error>> {
        let op_code: u16 = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[self.program_counter as usize + 1] as u16);
        self.execute_instruction(op_code)?;
        //decode opcode
        //execute opcode
        self.update_timers()?;
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
            keyboard: Keyboard::init(),
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

    pub fn set_keys(&mut self, keys: HashSet<char>){
        self.keyboard.set_state(keys);
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
    fn update_timers(&mut self) -> Result<(), Box<dyn Error>> {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        };
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                doBeep();
                self.sound_timer -= 1;
            }
        };
        Ok(())
    }

    


    fn execute_instruction(&mut self, op_code: u16) -> Result<(), Box<dyn Error>>{
        match op_code & 0xF000 {
            0x0000 => {
                match op_code & 0x000F {
                    0x0000 => {
                        // trust me the 0x0000 catching the 0x00E0 makes sense.  
                        // 0x00E0 Clears the screen
                        self.graphics_buffer = [0; 64*32];
                        self.program_counter += 2;
                    }
                    0x000E => {
                        // Returns from subroutine
                        self.program_counter = self.stack.pop().unwrap();
                    }
                    _ => println!("Unknown opcode [ 0x0000 block]: {}", op_code)
                }
            }
            0x1000 => {
                self.program_counter = op_code & 0x0FFF
            }
            0x2000 => { // calls subroutine at 0x0NNN
                self.stack.push(self.program_counter);
                self.program_counter = op_code & 0x0FFF;
            }
            0x3000 => { // 0x3xkk skip next instruction if Vx == kk
                if self.registers[(op_code & 0x0F00) as usize] == (op_code & 0x00FF) as u8 {
                    self.program_counter += 2;
                }
                self.program_counter +=2;
            }
            0x4000 => { // 0x4xkk skip next instruction if Vx != kk
                if self.registers[(op_code & 0x0F00) as usize] != (op_code & 0x0FF) as u8 {
                    self.program_counter +=2;
                }
                self.program_counter += 2;
            }
            0x8000 => { // contains a 1 - 7 + E  codes that do register math.  
                match op_code & 0x000F {
                    0x0004 => { // 0x8XY4  adds Value of VY to VX, VF is set to 1 if ther is a carry, 0 if not
                        if self.registers[((op_code & 0x00f0) >> 4) as usize] > (0xFF - self.registers[((op_code & 0xF000)) as usize]) {
                            self.registers[0xF] = 1;
                        }else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[((op_code & 0x0F00) >> 8 )as usize] += self.registers[(op_code & 0x00F0) as usize];
                        self.program_counter += 2;
                    }
                    _ => println!("Unknown opcode [0x8XYN]: {}",op_code)
                }
            }
            
            0xA000 => {
                    self.index_register = op_code & 0x0FFF;
                    self.program_counter += 2;
                
                }
            _ => println!("Not Implemented or not supported opcode")
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_instruction_should_catchOpCode() {
        println!("{}",0xF);
        
    }

}

fn doBeep() {
        
    }