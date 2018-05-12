#![allow(dead_code, unused_variables)]

use failure::Error;
use specs::{ReadStorage, System, World};
use std::io::{stdin, stdout, Write};
use termion;
use termion::cursor;
use termion::raw::IntoRawMode;

pub mod components;
use self::components::*;

struct RenderingSystem;
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Appearance>);

    fn run(&mut self, (pos, ap): Self::SystemData) {
        use specs::Join;

        // TODO: add error handling, don't reallocate stdout each time.
        let (_, term_height) = termion::terminal_size().expect("couldn't get terminal size");
        let mut stdout = stdout().into_raw_mode().expect("coudln't access stdout");

        for (p, a) in (&pos, &ap).join() {
            let y = term_height - p.y as u16;
            write!(stdout, "{}{}", cursor::Goto(p.x as u16, y), a.to_string())
                .expect("couldn't print to stdout");
        }
        stdout.flush().expect("failed flushing stdout");
    }
}

pub fn run_game() -> Result<(), Error> {
    // let mut stdout = stdout().into_raw_mode()?;
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Appearance>();

    let player = world
        .create_entity()
        .with(Position { x: 0, y: 2 })
        .with(Appearance::Player)
        .build();

    use specs::RunNow;
    let mut rendering_system = RenderingSystem;
    rendering_system.run_now(&world.res);

    stdin().read_line(&mut String::new())?;

    Ok(())
}
