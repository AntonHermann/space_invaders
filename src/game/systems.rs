use specs::{FetchMut, Join, ReadStorage, System, WriteStorage};
use std::io::{stdin, Stdout, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

use super::components::*;
use super::GameActive;

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

enum HorizontalMove {
    Left(usize),
    Right(usize),
}

pub struct PlayerInteractionSystem;
impl<'a> System<'a> for PlayerInteractionSystem {
    type SystemData = (WriteStorage<'a, Position>, FetchMut<'a, GameActive>);

    fn run(&mut self, (mut pos, mut ga): Self::SystemData) {
        let term_width = termion::terminal_size()
            .expect("couldn't get terminal width")
            .0 as usize;
        let mut move_horizontal = |mov: HorizontalMove| match mov {
            HorizontalMove::Left(dist) => {
                for p in (&mut pos).join() {
                    p.x = if dist > p.x { 0 } else { p.x - dist };
                }
            }
            HorizontalMove::Right(dist) => {
                for p in (&mut pos).join() {
                    p.x = if dist + p.x > term_width {
                        term_width
                    } else {
                        p.x + dist
                    };
                }
            }
        };

        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    *ga = GameActive(false);
                    break;
                }
                Key::Left => {
                    move_horizontal(HorizontalMove::Left(1));
                    break;
                }
                Key::Right => {
                    move_horizontal(HorizontalMove::Right(1));
                    break;
                }
                _ => {}
            }
        }
    }
}
