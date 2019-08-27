extern crate crossterm;

use std::error::Error;
use chip::Chip_8;
use crossterm::{terminal, Terminal, RawScreen, Crossterm, TerminalCursor, ClearType, AsyncReader};

mod chip;
fn main() -> Result<(), Box<dyn Error>> {

    // screen has to be in raw mode in order for the key presses not to be printed to the screen.
    let raw = RawScreen::into_raw_mode();
    let crossterm = Crossterm::new();
    crossterm.cursor().hide()?;

    let terminal = terminal();
    terminal.set_size(64,32)?;

    let mut input = crossterm.input().read_async();

    let mut chip = Chip_8::init()?;
    chip.load_program(String::from("test"))?;

    loop {
        // this must take ~17 ms or whatever is 60 fps.  
        chip.run_cycle()?;
        draw_graphics(&chip.get_graphics_buffer(), &terminal)?;
        if let Some(keys) = get_key_presses(input) {
            chip.set_keys(keys);
        };
    }

    Ok(())
}



fn draw_graphics(graphics_buffer: &[u8; 64*32], terminal: &Terminal) -> Result<(), Box<dyn Error>> {
    
//    terminal.clear(ClearType::All)?;  // Is this needed?
    let cursor = TerminalCursor::new();
    for x in 0..63 {
        for y in 0..31 {
            cursor.goto(x, y)?;
            print!("{}", graphics_buffer[(x as usize * y as usize)])
        }
    }

    

    Ok(())
}

fn get_key_presses(input_reader: &mut AsyncReader) -> Option<?> {
    // for each input in iterator, add the key pressed to the keyboard vec/ struct.  may wanna just map it to a u8 and bitwise
    input_reader.for_each(mut f: F)
}