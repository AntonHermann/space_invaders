use specs::prelude::*;
use std::sync::mpsc::Receiver;
use termion::event::Key;

use super::components::*;
use super::GameActive;
use super::weapon::Weapon;

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
        ReadStorage<'a, PlayerControls>,
        Write<'a, GameActive>,
    );

    fn run(&mut self, (mut velocity, mut weapon, pc, mut ga): Self::SystemData) {
        while let Ok(key) = self.rx.try_recv() {
            match key {
                Key::Char('q') => {
                    debug!("quit");
                    *ga = GameActive(false);
                    break;
                },
                Key::Esc => {
                    debug!("pause");
                    // TODO: implement pause mechanic
                }
                key => {
                    // loop players to match player control keys
                    for (velocity, _weapon, pc) in (&mut velocity, &mut weapon, &pc).join() {
                        if key == pc.key_move_left {
                            velocity.0 -= 1;
                        } else if key == pc.key_move_right {
                            velocity.0 += 1;
                        } else if key == pc.key_shoot {
                            // TODO: implement shooting mechanic
                        }
                    }
                }
            }
        }
    }
}