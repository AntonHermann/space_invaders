use specs::prelude::*;
use termion;
use super::*;

pub struct WeaponSystem;
impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Weapon>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, (entities, mut weapon, updater): Self::SystemData) {
        trace!("enter");
        for weapon in (&mut weapon).join() {
            trace!("weapon {:?}", weapon);
            // fire shots
            if let Some(pos) = weapon.shot.take() {
                let shot = updater.create_entity(&entities)
                    .with(pos)
                    .with(Appearance::Shot)
                    .with(Projectile::allied())
                    .build();
                info!("created shot: {:?}", shot);
            }
            // and decrease cooldown
            if weapon.current_cooldown > 0 {
                weapon.current_cooldown -= 1;
                trace!("decrease weapon cd to: {}", weapon.current_cooldown);
            }
        }
    }
}

pub struct BulletMovementSystem;
impl<'a> System<'a> for BulletMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Projectile>,
        Entities<'a>,
    );

    fn run(&mut self, (mut position, mut projectile, ents): Self::SystemData) {
        trace!("enter");

        let term_height = termion::terminal_size()
            .expect("couldn't get terminal height")
            .1 as usize;

        let mut bullet_reached_top = false;

        for (pos, proj, ent) in (&mut position, &mut projectile, &*ents).join() {
            trace!("bullet: {:?} {:?} {:?}", ent, proj, pos);
            match proj.ptype {
                ProjectileType::Allied => {
                    trace!("  {} < {}", pos.y, term_height);
                    if pos.y < term_height {
                        pos.y += 1;
                    } else {
                        // proj.remove_flag = true;
                        // bullet_reached_top = true;
                        let res = (&*ents).delete(ent);
                        debug!("tried to delete bullet, result: {:?}", res);
                    }
                },
                ProjectileType::Enemy  => {
                    // TODO: implement enemy bullet movement
                    unimplemented!()
                },
            }
        }
    }
}