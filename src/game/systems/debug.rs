use game::components::*;
use specs::prelude::*;

pub struct DebugSystem;
impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
    );

    fn run(&mut self, (pos, ap): Self::SystemData) {
        // trace!("enter");
        // for (pos, ap) in (&pos, &ap).join() {
        //     debug!("{:?} {:?}", pos, ap);
        // }
    }
}