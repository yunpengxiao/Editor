use std::io::{self, stdout, Write};            
use termion::event::Key;            
use termion::input::TermRead;            
use termion::raw::{IntoRawMode, RawTerminal};
use crate::editor::Position;
use termion::color;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    
    pub fn size(&self) -> &Size {
        &self.size
    }
    
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    
    pub fn cursor_position(p: &Position) {
        let x = p.x.saturating_add(1);
        let y = p.y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }
    
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}