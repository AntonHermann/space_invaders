use failure::Error;
use specs::{DispatcherBuilder, World};
use std::{thread, time};

mod components;
mod player_interaction;
mod player_movement;
mod display;
mod weapon;
mod input_event_handler;

mod debug_system;

use self::components::*;
use self::player_interaction::*;
use self::player_movement::*;
use self::display::*;
use self::weapon::*;
use self::input_event_handler::*;

use self::debug_system::*;

#[derive(Clone, Debug)]
pub enum VDirection {
    Up,
    Down,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct GameActive(bool);

pub fn run_game() -> Result<(), Error> {
    let rx = input_events();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Appearance>();
    world.register::<PlayerControls>();
    world.register::<Weapon>();
    world.register::<Projectile>();
    world.add_resource(GameActive(true));

    init_player(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(RenderingSystem::new(), "rendering_system", &[])
        .with(PlayerInteractionSystem::new(rx), "player_interaction_system", &[])
        .with(PlayerMovementSystem, "player_movement_system", &["player_interaction_system"])
        .with(WeaponSystem, "weapon_system", &["player_interaction_system"])
        .with(BulletMovementSystem, "bullet_movement_system", &[])
        .with(DebugSystem, "debug_system", &[])
        .build();

    let sleep_duration = time::Duration::from_millis(10);
    // let sleep_duration = time::Duration::from_millis(10000);

    // GAME LOOP
    loop {
        trace!("tick");
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        if world.read_resource::<GameActive>().0 == false {
            break;
        }
        thread::sleep(sleep_duration);
    }

    Ok(())
}

fn init_player(world: &mut World) {
    let p = world
        .create_entity()
        .with(Position { x: 5, y: 2 })
        .with(Velocity::idle())
        .with(Appearance::Player)
        .with(PlayerControls::default_player())
        .with(Weapon::default_player())
        .build();
    info!("created player: {:?}", p);
}
