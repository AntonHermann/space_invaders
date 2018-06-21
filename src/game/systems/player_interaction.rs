use specs::prelude::*;
use std::sync::mpsc::Receiver;
use termion::event::Key;

use game::components::*;
use game::GameActive;

pub struct PlayerInteractionSystem {
    rx: Receiver<Key>,
}
impl PlayerInteractionSystem {
    pub fn new(rx: Receiver<Key>) -> Self {
        PlayerInteractionSystem {
            rx
        }
    }
}
impl<'a> System<'a> for PlayerInteractionSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Weapon>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerControls>,
        Write<'a, GameActive>,
    );

    fn run(&mut self, (mut velocity, mut weapon, pos, pc, mut ga): Self::SystemData) {
        trace!("enter");
        while let Ok(key) = self.rx.try_recv() {
            trace!("key: {:?}", key);
            match key {
                Key::Char('q') | Key::Ctrl('q') => {
                    debug!("quit");
                    *ga = GameActive(false);
                    break;
                },
                Key::Esc => {
                    debug!("pause");
                    // TODO: implement pause mechanic
                    *ga = GameActive(false);
                    break;
                }
                key => {
                    // loop players to match player control keys
                    for (velocity, weapon, pos, pc) in (&mut velocity, &mut weapon, &pos, &pc).join() {
                        if key == pc.key_move_left {
                            trace!("key_left");
                            velocity.0 -= 1;
                        } else if key == pc.key_move_right {
                            trace!("key_right");
                            velocity.0 += 1;
                        } else if key == pc.key_shoot {
                            trace!("key_shoot");
                            weapon.try_shoot(pos);
                        }
                    }
                }
            }
        }
    }
}