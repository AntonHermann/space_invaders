use failure::Error;
use specs::{DispatcherBuilder, World};
use std::io::stdout;
use std::{thread, time};
use termion::raw::IntoRawMode;

pub mod components;
pub mod player_interaction;
mod display;
use self::components::*;
use self::player_interaction::*;
use self::display::*;

pub struct GameActive(bool);

pub fn run_game() -> Result<(), Error> {
    let mut stdout = stdout().into_raw_mode()?;
    clear_screen(&mut stdout)?;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Appearance>();
    world.add_resource(stdout);
    world.add_resource(GameActive(true));

    init_player(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .add(RenderingSystem, "rendering_system", &[])
        .add(PlayerInteractionSystem, "player_interaction_system", &[])
        .build();

    let sleep_duration = time::Duration::from_millis(10);

    // GAME LOOP
    loop {
        dispatcher.dispatch(&mut world.res);
        if world.read_resource::<GameActive>().0 == false {
            break;
        }
        thread::sleep(sleep_duration);
    }

    Ok(())
}

fn init_player(world: &mut World) {
    world.create_entity()
        .with(Position { x: 0, y: 2 })
        .with(Appearance::Player)
        .build();
}
