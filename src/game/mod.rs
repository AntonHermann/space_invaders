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
    let mut world = init_world()?;
    init_player(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .add(RenderingSystem, "rendering_system", &[])
        .add(PlayerInteractionSystem, "player_interaction_system", &[])
        .build();

    let sleep_duration = time::Duration::from_millis(10);

    loop {
        dispatcher.dispatch(&mut world.res);
        if world.read_resource::<GameActive>().0 == false {
            break;
        }
        thread::sleep(sleep_duration);
    }

    Ok(())
}

fn init_world() -> Result<World, Error> {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Appearance>();
    world.add_resource(stdout().into_raw_mode()?);
    world.add_resource(GameActive(true));
    Ok(world)
}

fn init_player(world: &mut World) {
    world.create_entity()
        .with(Position { x: 0, y: 2 })
        .with(Appearance::Player)
        .build();
}