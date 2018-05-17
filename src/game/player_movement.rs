use specs::prelude::*;
use termion;

use super::components::*;

pub struct PlayerMovementSystem;
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Appearance>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, PlayerControls>,
    );

    fn run(&mut self, (ap, mut pos, mut vel, pc): Self::SystemData) {
        trace!("enter");

        let term_width = termion::terminal_size()
            .expect("couldn't get terminal width")
            .0 as usize;

        // _pc is only neccessary to ensure we're moving a player
        for (ap, pos, vel, _pc) in (&ap, &mut pos, &mut vel, &pc).join() {
            trace!("player  {:?} {:?} {:?}", ap, pos, vel);
            let new_x = pos.x as isize + vel.0;

            let w = ap.get_width() / 2;
            let left_bound  = 1 + w;
            let right_bound = term_width - w;

            pos.x = if new_x <= left_bound as isize {
                left_bound // left bound
            } else if new_x >= right_bound as isize {
                right_bound // right bound
            } else {
                new_x as usize // inside bounds
            };
            vel.0 = 0; // reset velocity
            trace!("player_ {:?} {:?} {:?}", ap, pos, vel);
        }
    }
}