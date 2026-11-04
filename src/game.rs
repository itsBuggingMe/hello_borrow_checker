use std::io::{stdout, Write};
use crossterm::execute;
use crossterm::terminal::SetSize;

pub struct ConsoleGame {
    back_width: usize,
    back_height: usize,
    stride: usize,
    back_buf: Vec<u8>,
    empty_line: Vec<u8>,
}

impl ConsoleGame {
    const PIXEL_CHAR_SIZE:usize = 25;

    pub fn new(back_width: usize, back_height: usize) -> ConsoleGame {
        let empty = "\x1b[38;2;000;000;000m██".repeat(back_width);

        ConsoleGame{
            back_width,
            back_height,
            stride: back_width * Self::PIXEL_CHAR_SIZE + 1,
            back_buf: ((empty.clone() + "\n").repeat(back_height - 1) + &*empty.clone())
                .bytes()
                .collect(),
            empty_line: empty
                .bytes()
                .collect(),
        }
    }

    pub fn init(&self) {
        execute!(stdout(), SetSize(self.back_width as u16 * 2, self.back_height as u16))
            .unwrap()
    }

    pub fn flush(&self) {
        if let Ok(buff) = std::str::from_utf8(&self.back_buf) {
            print!("\x1B[1;1H{}\x1b[?25l", buff)
        }

        stdout().flush().unwrap();
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.back_width {
            return;
        }
        if y >= self.back_height {
            return;
        }

        let pix_start = x * Self::PIXEL_CHAR_SIZE + y * self.stride + 7;
        write!(&mut self.back_buf[pix_start + 0..pix_start + 3],  "{r:03}").unwrap();
        write!(&mut self.back_buf[pix_start + 4..pix_start + 7],  "{g:03}").unwrap();
        write!(&mut self.back_buf[pix_start + 8..pix_start + 11], "{b:03}").unwrap();
    }

    pub fn write_text(&self, x: usize, y: usize, text: String) {
        print!("\x1b[0m\x1b[{};{}H{text}", y + 1, x * 2 + 1);
        stdout().flush().unwrap();
    }

    pub fn clear_back_buffer(&mut self) {
        for row in 0..self.back_height {
            let row_start = row * self.stride;
            self.back_buf[row_start..(row_start + Self::PIXEL_CHAR_SIZE * self.back_width)]
                .copy_from_slice(self.empty_line.as_slice())
        }
    }
}