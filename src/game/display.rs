use std::io::{Write, Stdout};
use std::io::Error as IoError;
use termion;
use termion::{clear, cursor};
use termion::raw::RawTerminal;
use specs::{System, ReadStorage, FetchMut, Join};

use super::components::*;

pub struct RenderingSystem;
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
        FetchMut<'a, RawTerminal<Stdout>>,
    );

    fn run(&mut self, (pos, ap, mut stdout): Self::SystemData) {
        // TODO: add error handling
        let (_, term_height) = termion::terminal_size().expect("couldn't get terminal size");

        for (p, a) in (&pos, &ap).join() {
            let y = term_height - p.y as u16;
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto(p.x as u16, y),
                clear::All,
                a.to_string()
            ).expect("couldn't print to stdout");
        }
        stdout.flush().expect("failed flushing stdout");
    }
}

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), IoError> {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide)
}