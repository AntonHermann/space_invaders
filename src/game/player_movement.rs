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
        let term_width = termion::terminal_size()
            .expect("couldn't get terminal width")
            .0 as usize;

        for (ap, pos, vel, pc) in (&ap, &mut pos, &mut vel, &pc).join() {
            let new_x = pos.x as isize + vel.0;
            let right_bound = term_width - ap.get_width() + 1;
            pos.x = if new_x < 0 {
                0 // left bound
            } else if new_x >= right_bound as isize {
                right_bound // right bound
            } else {
                new_x as usize // inside bounds
            };
            vel.0 = 0; // reset velocity
        }
    }
}