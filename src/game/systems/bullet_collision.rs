use specs::prelude::*;
use game::components::*;

pub struct BulletCollisionSystem;
impl<'a> System<'a> for BulletCollisionSystem {
    type SystemData = (
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EnemyFlag>,
        Entities<'a>,
    );
    fn run(&mut self, (proj, pos, enemy, ents): Self::SystemData) {
        trace!("enter");
        let mut bullets = Vec::new();
        // loop through the bullets
        for (b_pos, b_type, b_ent) in (&pos, &proj, &*ents).join() {
            // only process allied bullets
            if b_type != &Projectile::Allied { continue; }

            bullets.push((b_pos, b_ent));
        }

        // loop through enemies
        for (e_pos, _, e_ent) in (&pos, &enemy, &*ents).join() {
            // TODO: implement proper box collision checking
            for b in &bullets {
                if e_pos == b.0 {
                    // COLLISION!
                    info!("collision at {:?}", e_pos);
                    (&*ents).delete(b.1)  .expect("failed removing shot");
                    (&*ents).delete(e_ent).expect("failed removing enemy");
                    continue;
                }
            }
        }
    }
}