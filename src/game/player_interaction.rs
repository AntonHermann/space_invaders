use specs::{FetchMut, Join, ReadStorage, System, WriteStorage};
use std::io::stdin;
use termion;
use termion::event::Key;
use termion::input::TermRead;

use super::components::*;
use super::GameActive;

enum HorizontalMove {
    Left(usize),
    Right(usize),
}

pub struct PlayerInteractionSystem;
impl<'a> System<'a> for PlayerInteractionSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
        FetchMut<'a, GameActive>,
    );

    fn run(&mut self, (mut pos, ap, mut ga): Self::SystemData) {
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
                for (p, a) in (&mut pos, &ap).join() {
                    let width = a.get_width();
                    p.x = if dist + p.x >= term_width - width {
                        term_width - width
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