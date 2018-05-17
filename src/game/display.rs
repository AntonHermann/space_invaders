use std::io::{Stdout, stdout, Write};
use termion;
use termion::{clear, cursor};
use termion::raw::{RawTerminal, IntoRawMode};
use specs::prelude::*;

use super::components::*;

fn flush(rt: &mut RawTerminal<Stdout>) {
    rt.flush().expect("failed flushing stdout");
}
fn clear_screen(rt: &mut RawTerminal<Stdout>) {
    write!(rt, "{}{}", termion::clear::All, termion::cursor::Hide)
        .expect("couldn't write to stdout");
}

pub struct RenderingSystem {
    stdout: RawTerminal<Stdout>,
}
impl RenderingSystem {
    pub fn new() -> Self {
        let mut stdout = stdout().into_raw_mode().expect("couldn't get stdout");
        clear_screen(&mut stdout);

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
        trace!("enter");
        // TODO: add error handling
        let (_, term_height) = termion::terminal_size().expect("couldn't get terminal size");

        // write!(self.stdout, "{}", clear::All).expect("couldn't clear screen");
        clear_screen(&mut self.stdout);
        for (p, a) in (&pos, &ap).join() {
            trace!("rendering {:?} {:?}", a, p);
            let y = term_height - p.y as u16;
            let w = a.get_width() / 2;
            let x = if p.x > w { p.x - w } else { 1 }; // make sure x is 1 or greater
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(x as u16, y),
                a.to_string()
            ).expect("couldn't print to stdout");
        }
        flush(&mut self.stdout);
    }
}