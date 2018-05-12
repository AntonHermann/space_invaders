use failure::Error;
use specs::{DispatcherBuilder, World};
use std::io::stdout;
use std::{thread, time};
use termion::raw::IntoRawMode;

pub mod components;
pub mod systems;
use self::components::*;
use self::systems::*;

pub struct GameActive(bool);

pub fn run_game() -> Result<(), Error> {
    // let mut stdout = stdout().into_raw_mode()?;
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Appearance>();
    world.add_resource(stdout().into_raw_mode()?);
    world.add_resource(GameActive(true));

    let _player = world
        .create_entity()
        .with(Position { x: 0, y: 2 })
        .with(Appearance::Player)
        .build();

    // let mut rendering_system = RenderingSystem;
    // let mut player_interaction_system = PlayerInteractionSystem;
    let mut dispatcher = DispatcherBuilder::new()
        .add(RenderingSystem, "rendering_system", &[])
        .add(PlayerInteractionSystem, "player_interaction_system", &[])
        // .build_async(world);
        .build();

    let sleep_duration = time::Duration::from_millis(10);

    // dispatcher.dispatch(&mut world.res);
    // stdin().read_line(&mut String::new())?;

    loop {
        dispatcher.dispatch(&mut world.res);
        // dispatcher.dispatch();
        if *(&world.read_resource::<GameActive>().0) == false {
            break;
        }
        thread::sleep(sleep_duration);
    }

    Ok(())
}
