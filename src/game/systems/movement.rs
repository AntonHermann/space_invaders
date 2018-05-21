use specs::prelude::*;
use termion;

use game::components::*;

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

pub struct EnemyMovementSystem {
    moving_right: bool,
}
impl EnemyMovementSystem {
    pub fn new() -> Self {
        EnemyMovementSystem {
            moving_right: true,
        }
    }
}
impl<'a> System<'a> for EnemyMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, EnemyFlag>,
    );
    fn run(&mut self, (mut pos, e_flag): Self::SystemData) {
        use ::std::cmp::*;
        trace!("enter");

        // let (positions_x, positions_y): (Vec<_>, Vec<_>) = (&pos, &e_flag).join()
        //     .map(|(pos, _)| (pos.x, pos.y))
        //     .unzip();
        // let top_bound = positions_y.iter().max(); // max because y counts from bottom
        // let left_bound = positions_x.iter().min();
        // let right_bound = positions_x.iter().max();
        // eprintln!("t: {:?}, l: {:?}, r: {:?}", top_bound, left_bound, right_bound);

        let (top_bound, left_bound, right_bound) = (&pos, &e_flag).join()
            // build iterator of (y, x, x) tuples
            .map(|(pos, _)| (pos.y, pos.x, pos.x))
            // fold the tuples to get a (max_y, min_x, max_x) tuple
            // this could possibly result in (0, MAX, 0) if there are no
            // enemies left, but this should never happen because then the game is over.
            .fold((0, ::std::usize::MAX, 0), |(t1, l1, r1), (t2, l2, r2)| {
                (max(t1, t2), min(l1, l2), max(r1, r2))
            });
        debug!("bounds: t: {}, l: {}, r: {}", top_bound, left_bound, right_bound);

        if self.moving_right {
            let (term_width, _term_height) = termion::terminal_size().expect("couldn't get terminal size");
            if right_bound + 1 >= term_width as usize {
                // reached right border, change direction and move 1 down
                self.moving_right = false;
                for (pos, _) in (&mut pos, &e_flag).join() {
                    pos.y -= 1;
                }
            } else {
                // move 1 to the right
                for (pos, _) in (&mut pos, &e_flag).join() {
                    pos.x += 1;
                }
            }
        } else {
            if left_bound as isize - 1 <= 0 {
                // reached left border, change direction and move 1 down
                self.moving_right = true;
                for (pos, _) in (&mut pos, &e_flag).join() {
                    pos.y -= 1;
                }
            } else {
                // move 1 to the left;
                for (pos, _) in (&mut pos, &e_flag).join() {
                    pos.x -= 1;
                }
            }
        };

    }
}