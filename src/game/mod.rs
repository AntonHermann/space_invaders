use failure::Error;
use specs::{DispatcherBuilder, World};
use std::{thread, time};
use std::sync::atomic::Ordering;

mod components;
mod systems;
mod input_event_handler;

use self::components::*;
use self::systems::*;
use self::input_event_handler::*;

const ENEMY_COUNT_X:  usize = 6;
const ENEMY_COUNT_Y:  usize = 5;
const ENEMY_COUNT:    usize = ENEMY_COUNT_X * ENEMY_COUNT_Y;
const ENEMY_SPACE_X:  usize = 7;
const ENEMY_SPACE_Y:  usize = 3;
const ENEMY_OFFSET_X: usize = 5;
const ENEMY_OFFSET_Y: usize = 5;

const PLAYER_STR: &'static str = "/O\\";
const ENEMY_STR:  &'static str = "U";
const SHOT_STR:   &'static str = "|";

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
    world.register::<EnemyFlag>();
    world.add_resource(GameActive(true));

    init_player(&mut world);
    init_enemies(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(RenderingSystem::new(), "rendering_system", &[])
        .with(PlayerInteractionSystem::new(rx), "player_interaction_system", &[])
        .with(PlayerMovementSystem, "player_movement_system", &["player_interaction_system"])
        .with(EnemyMovementSystem::new(), "enemy_movement_system", &[])
        .with(WeaponSystem, "weapon_system", &["player_interaction_system"])
        .with(BulletMovementSystem, "bullet_movement_system", &[])
        .with(BulletCollisionSystem, "bullet_collision_system",
            &["enemy_movement_system", "bullet_movement_system"])
        .with(DebugSystem, "debug_system", &[])
        .build();

    // frames   s      ms
    //   30     1     1000
    //   10     1/3   1000/3
    //    1     1/30  100/3
    let target_update_rate = time::Duration::from_millis(100) / 3;

    // GAME LOOP
    loop {
        trace!("tick");
        let start = time::Instant::now();
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        if world.read_resource::<GameActive>().0 == false {
            break;
        }
        let elapsed = start.elapsed();
        thread::sleep(target_update_rate - elapsed);
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

fn init_enemies(world: &mut World) {
    use termion::terminal_size;
    let (_, term_height) = terminal_size().expect("couldn't get terminal size");

    for x in 0..ENEMY_COUNT_X {
        for y in 0..ENEMY_COUNT_Y {
            let e = world
                .create_entity()
                .with(Position {
                    x: ENEMY_OFFSET_X + x * ENEMY_SPACE_X,
                    y: term_height as usize - (ENEMY_OFFSET_Y + y * ENEMY_SPACE_Y),
                })
                .with(Velocity::idle())
                .with(Appearance::Enemy)
                .with(EnemyFlag)
                .build();
            info!("created enemy [{},{}]: {:?}", x, y, e);
        }
    }
}