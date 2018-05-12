use failure::Error;
use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;
use termion::cursor;
use termion;
use specs::{Component, VecStorage, World, System, ReadStorage};

#[derive(Clone,Debug)]
struct Position {
    pub x: usize,
    pub y: usize,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
enum Appearance {
    Player,
    Enemy,
    Other(String),
}
use std::borrow::Cow;
impl Appearance {
    fn to_string(&self) -> Cow<str> {
        match self {
            Appearance::Player => Cow::Borrowed("/O\\"),
            Appearance::Enemy  => Cow::Borrowed("U"),
            Appearance::Other(s) => Cow::Borrowed(s),
        }
    }
}
impl Component for Appearance {
    type Storage = VecStorage<Self>;
}

struct RenderingSystem;
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Appearance>);

    fn run(&mut self, (pos, ap): Self::SystemData) {
        use specs::Join;
        use termion::cursor;

        // TODO: add error handling, don't reallocate stdout each time.
        let (_, term_height) = termion::terminal_size().expect("couldn't get terminal size");
        let mut stdout = stdout().into_raw_mode().expect("coudln't access stdout");

        for (p,a) in (&pos, &ap).join() {
            let y = term_height - p.y as u16;
            write!(stdout, "{}{}", cursor::Goto(p.x as u16, y), a.to_string());
        }
        stdout.flush().expect("failed flushing stdout");
    }
}

pub fn run_game() -> Result<(), Error> {
    // let mut stdout = stdout().into_raw_mode()?;
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Appearance>();

    let player = world.create_entity()
        .with(Position {x: 0, y: 2})
        .with(Appearance::Player).build();

    use specs::RunNow;
    let mut rendering_system = RenderingSystem;
    rendering_system.run_now(&world.res);

    stdin().read_line(&mut String::new())?;

    Ok(())
}