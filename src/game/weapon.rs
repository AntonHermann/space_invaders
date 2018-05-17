use specs::prelude::*;
use std::time::{Instant, Duration};
use super::*;

pub struct Weapon {
    dir: VDirection,
    last_shot: Instant,
    cooldown: Duration,
}
impl Component for Weapon {
    type Storage = VecStorage<Self>;
}
impl Weapon {
    pub fn new(dir: VDirection, cooldown: Duration) -> Self {
        Weapon {
            dir, last_shot: Instant::now(), cooldown
        }
    }
}

pub struct WeaponSystem;
impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        // Entities<'a>,
        // ReadStorage<'a, Position>,
        // WriteStorage<'a, PlayerControls>,
        // WriteStorage<'a, Weapon>,
        // Read<'a, LazyUpdate>
    );

    fn run(&mut self, _: Self::SystemData) {
        // TODO: implement weapon system
    }
    // fn run(&mut self, (entities, pos, mut paq, mut weapon, updater): Self::SystemData) {
    //     for (position, paq, w) in (&pos, &mut paq, &mut weapon).join() {
    //         // if weapon isn't on cooldown
    //         if w.last_shot.elapsed() >= w.cooldown {
    //             // ... and front action is shoot
    //             if paq.pop_if_eq(&PlayerAction::Shoot) {
    //                 w.last_shot = Instant::now();
    //                 let shot = updater
    //                     .create_entity(&entities)
    //                     .with(position.clone())
    //                     .with(Appearance::Shot)
    //                     .build();
    //                 info!("created shot: {:?}", shot);
    //             }
    //         }
    //     }
    // }
}