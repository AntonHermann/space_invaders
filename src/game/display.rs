use std::io::{Stdout, stdout, Write};
use termion;
use termion::{clear, cursor};
use termion::raw::{RawTerminal, IntoRawMode};
// use specs::{System, ReadStorage, FetchMut, Join};
use specs::prelude::*;

use super::components::*;

pub struct Terminal(RawTerminal<Stdout>);
impl Default for Terminal {
    fn default() -> Self {
        Terminal(stdout().into_raw_mode().expect("couldn't get stdout"))
    }
}
impl ::std::ops::Deref for Terminal {
    type Target = RawTerminal<Stdout>;
    fn deref(&self) -> &RawTerminal<Stdout> {
        &self.0
    }
}
impl ::std::ops::DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut RawTerminal<Stdout> {
        &mut self.0
    }
}
impl Terminal {
    pub fn flush(&mut self) {
        self.0.flush().expect("failed flushing stdout");
    }
    pub fn clear_screen(&mut self) {
        write!(self.0, "{}{}", termion::clear::All, termion::cursor::Hide)
            .expect("couldn't write to stdout");
    }
}

pub struct RenderingSystem {
    stdout: Terminal,
}
impl RenderingSystem {
    pub fn new() -> Self {
        let mut stdout = Terminal::default();
        stdout.clear_screen();

        RenderingSystem {
            stdout
        }
    }
}
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
    );

    fn run(&mut self, (pos, ap): Self::SystemData) {
        // TODO: add error handling
        let (_, term_height) = termion::terminal_size().expect("couldn't get terminal size");

        for (p, a) in (&pos, &ap).join() {
            let y = term_height - p.y as u16;
            write!(
                self.stdout,
                "{}{}{}",
                cursor::Goto(p.x as u16, y),
                clear::All,
                a.to_string()
            ).expect("couldn't print to stdout");
        }
        self.stdout.flush();
    }
}